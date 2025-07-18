# Rust AF_XDP Demo

Rust AF_XDP demo using Aya for high-performance packet processing. Includes AWS setup via Ansible and containerized deployment options.

[![Rust](https://img.shields.io/badge/Rust-1.74%2B-orange)](https://www.rust-lang.org/)
[![eBPF](https://img.shields.io/badge/eBPF-AF__XDP-blue)](https://ebpf.io)
[![AWS](https://img.shields.io/badge/AWS-EC2-success)](https://aws.amazon.com/ec2/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

---

### Overview
`rust-afxdp-demo` provides a **complete solution** for:
- High-performance packet processing using **Rust + eBPF**
- **AF_XDP zero-copy networking**
- **AWS EC2 automation with Ansible**
- **Container-based deployment**

It uses the Rust [Aya](https://github.com/aya-rs/aya) library for writing **safe Rust eBPF programs**, avoiding unsafe C code.

---

### Repository Structure

```shell
rust-afxdp-demo/
├── ansible/
│   ├── inventory/hosts.ini
│   ├── playbooks/
│   │   ├── 01-create-vpc.yml
│   │   ├── 02-launch-ec2.yml
│   │   ├── 03-setup-instance.yml
│   │   └── 04-install-ebpf-tools.yml
├── aya-ebpf/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── xdp_filter.rs
│       └── common.rs
├── container/
│   ├── Dockerfile
│   └── entrypoint.sh
├── diagrams/
│   ├── architecture.puml        # PlantUML file
│   └── architecture.png         # Rendered diagram (optional)
├── scripts/
│   ├── build.sh
│   ├── run-test.sh
│   └── cleanup.sh
├── docs/
│   ├── AWS_SETUP.md
│   ├── EBPF_AYA_AF_XDP.md
│   ├── CONTAINER_SETUP.md
│   └── TESTING_GUIDE.md
└── .github/
    └── workflows/ci.yml

```

---

### Architecture Diagram  

![Architecture](diagrams/architecture.png)  

---

### Prerequisites
* AWS CLI and credentials configured  
* Ansible installed locally  
* Rust 1.74+, cargo xtask, bpf-linker  
* Linux kernel headers on EC2 instance  
* Podman (optional for container tests)  

---

### Deployment Steps  
**Step 1: Provision AWS Infrastructure**  
* Creates **VPC, subnets, Internet Gateway, security groups**  
* Deploys an **ENA-enabled EC2 instance** (e.g., `c5n.large`)
* Sets up **SSH access and Ansible inventory**  

Run: 
```shell
cd ansible
ansible-playbook playbooks/01-create-vpc.yml
ansible-playbook playbooks/02-launch-ec2.yml

```