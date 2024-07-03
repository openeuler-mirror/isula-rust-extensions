
// Copyright (c) 2024 Huawei Technologies Co.,Ltd. All rights reserved.
//
// isula-rust-extensions is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use std::collections::HashMap;
use std::io::{Write, Read};
use std::os::unix::net::UnixStream;
use std::sync::{Mutex, Arc, Once};
use std::thread;

use crate::nri::error::{Result, Error};


const RESERVED_CONN_ID: ConnId = 0;
type ConnId = u32;

// Mux is implemented as a simple multiplexer that forwards data between a trunk
// connection and multiple connections. The trunk connection is used to receive
// data from the outside world and the connections are used to send data to the
// outside world.
// The trunk connection receives data in the following format:
//   - 4 bytes: connection id
//   - 4 bytes: data length
//   - data
// The mux forwards the data to the connection with the specified id.
pub struct Mux {
    trunk: UnixStream,
    write_lock: Mutex<()>,
    conns: Mutex<HashMap<ConnId, UnixStream>>,
    close_once: Once,
}

impl Mux {
    pub fn new(trunk: UnixStream) -> Mux {
        Mux {
            trunk: trunk,
            write_lock: Mutex::new(()),
            conns: Mutex::new(HashMap::new()),
            close_once: Once::new(),
        }
    }

    // add conn to mux with id, and start a new thread to read data from the conn
    // and constantly write data with conn id and data length to the trunk
    pub fn add_conn(self: Arc<Self>, id: ConnId, mut stream: UnixStream) -> Result<()> {
        if id == RESERVED_CONN_ID {
            return Err(Error::InvalidArgument("conn id is reserved".to_string()));
        }

        {
            let mut conns = self.conns.lock()
                .map_err(|e| Error::Other(format!("lock error: {}", e)))?;
            if conns.contains_key(&id) {
                return Err(Error::InvalidArgument("conn id already exists".to_string()));
            }
            conns.insert(id, stream.try_clone()?);
        }

        thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                match stream.read(&mut buffer) {
                    Ok(cnt) => {
                        if cnt == 0 {
                            continue;
                        }
                        // println!("Conn {} Read {} bytes", id, cnt);
                        let mut hdr = [0; 8];
                        hdr[0..4].copy_from_slice(&id.to_be_bytes());
                        hdr[4..8].copy_from_slice(&(cnt as u32).to_be_bytes());

                        let _unused = self.write_lock.lock().unwrap();
                        let mut trunk = self.trunk.try_clone().unwrap();
                        if trunk.write_all(hdr.as_slice())
                            .and_then(|_| trunk.write_all(&buffer[0..cnt])).is_err() {
                            println!("isula_rust_extensions::conn_reader: trunk write error");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("isula_rust_extensions::conn_reader: conn {} Read error: {}", id, e);
                        break;
                    },
                }
            }

        });

        Ok(())
    }


    // trunk reader start a new thread to read data from trunk and forward it to the
    // corresponding connection
    pub fn trunk_reader(self: Arc<Self>) {
        thread::spawn(move || {
            const HEADER_LEN: usize = 8;
            loop {
                let mut hdr: [u8; HEADER_LEN] = [0; HEADER_LEN];
                let mut trunk = self.trunk.try_clone().unwrap();
                if let Err(e) = trunk.read_exact(&mut hdr) {
                    if self.close_once.is_completed() {
                        break;
                    }
                    self.close();
                    println!("isula_rust_extensions::trunk_reader: trunk read error: {}", e);
                    break;
                }
                // println!("Trunk read header: {:?}", hdr);
                let cid = u32::from_be_bytes(hdr[0..4].try_into().unwrap());
                let cnt = u32::from_be_bytes(hdr[4..8].try_into().unwrap());
                let mut buffer = vec![0; cnt as usize];
                if let Err(e) = trunk.read_exact(&mut buffer) {
                    self.close();
                    println!("isula_rust_extensions::trunk_reader: trunk read error: {}", e);
                    break;
                }
                // println!("Trunk read buffer: {:?}", buffer);

                if let Some(stream) = self.clone().conns.lock().unwrap().get_mut(&cid) {
                    if let Err(e) = stream.write_all(&buffer) {
                        self.close();
                        println!("isula_rust_extensions::trunk_reader: conn {} write error: {}", cid, e);
                        break;
                    }
                } else {
                    println!("isula_rust_extensions::trunk_reader: conn {} not found", cid);
                }
            }
        });
    }

    pub fn close(self: Arc<Self>) {
        self.close_once.call_once(|| {
            let mut conns = self.conns.lock().unwrap();
            for (_, stream) in conns.drain() {
                let _unused = stream.shutdown(std::net::Shutdown::Both);
            }
            let _unused = self.trunk.shutdown(std::net::Shutdown::Both);
        });
    }

}
