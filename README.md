# minitrace

A mini trace PHP extension written in Rust.

一个使用Rust编写的迷你trace PHP扩展。

# 简介

对于一些单体小项目的开发，引入CAT或者Skywalking这样的大型Trace系统显然有些“杀鸡用牛刀”，为其搭建服务端就是一件比较耗时的工作。但是我们又想享受到这类Trace系统的可观测性给开发过程带来的便利性，那么`minitrace`是你一个更好的选择。

`minitrace`将trace数据输出到本地文件而非远程服务端。