# iron-tunnel

A small learning project implementing SSH from scratch in Rust. The goal is to understand both Rust and the SSH protocol by building a minimal but secure SSH clone.

## Development Status

VERY Early development, barely proof-of-concept stage

## Features Checklist

### Basic Protocol
- [ ] Version exchange
- [ ] Binary packet protocol
- [ ] Message parsing and encoding
- [ ] Basic client/server communication

### Security
- [ ] Key exchange (KEX)
- [ ] Host key verification
- [ ] Session key generation
- [ ] Encryption/decryption
- [ ] Message authentication (MAC)

### Authentication
- [ ] Public key authentication
- [ ] Password authentication
- [ ] User management

### Connection
- [ ] Channel multiplexing
- [ ] Terminal handling
- [ ] PTY allocation
- [ ] Shell session
- [ ] Port forwarding

### Additional Features
- [ ] Connection keep-alive
- [ ] Compression
- [ ] Key re-exchange
- [ ] Session resumption

## License
MIT License - See LICENSE file for details
