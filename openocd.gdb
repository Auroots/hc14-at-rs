# 连接到远程调试器，端口号 3333
target extended-remote :3333

# 打印解析后的汇编符号
set print asm-demangle on

# 设置断点以检测未处理的异常、硬错误和 panic
break DefaultHandler
break HardFault
break rust_begin_unwind

# 启用半主机调试
monitor arm semihosting enable

# 载入程序到目标设备
load

# 启动程序，但立即暂停处理器
# stepi

# 启动程序
continue