# Crabby OS

一个基于 Rust 的简易 RISC-V 操作系统项目，可在 macOS M4 上开发并在 QEMU 上运行。

## 项目结构

```
my-riscv-os/
├── bootloader/     # 启动加载器
├── kernel/         # 操作系统内核
├── syscall/        # 系统调用接口
├── fs/             # 文件系统
├── user-programs/  # 用户态程序
├── scripts/        # 构建和运行脚本
└── Makefile        # 项目构建文件
```

## 开发环境要求

- macOS (支持 M4 芯片)
- Rust 工具链
- QEMU RISC-V 模拟器
- RISC-V 交叉编译工具链

## 快速开始

1. 设置开发环境：
```bash
make setup
```

2. 构建项目：
```bash
make build
```

3. 在 QEMU 中运行：
```bash
make run
```

## 模块说明

### Bootloader
负责系统启动和内核加载。

### Kernel
操作系统核心，包含：
- 内存管理
- 进程调度
- 中断处理
- 系统调用处理

### Syscall
定义系统调用接口和错误处理。

### FS
简单的文件系统实现。

### User Programs
用户态测试程序。
