<!-- README-zh-CN.md -->


```

███╗   ███╗ ██████╗ ███╗   ██╗ ██████╗       ██████╗ ██╗ █████╗  ██████╗ ██████╗  █████╗ ███╗   ███╗
████╗ ████║██╔═══██╗████╗  ██║██╔═══██╗      ██╔══██╗██║██╔══██╗██╔════╝ ██╔══██╗██╔══██╗████╗ ████║
██╔████╔██║██║   ██║██╔██╗ ██║██║   ██║█████╗██║  ██║██║███████║██║  ███╗██████╔╝███████║██╔████╔██║
██║╚██╔╝██║██║   ██║██║╚██╗██║██║   ██║╚════╝██║  ██║██║██╔══██║██║   ██║██╔══██╗██╔══██║██║╚██╔╝██║
██║ ╚═╝ ██║╚██████╔╝██║ ╚████║╚██████╔╝      ██████╔╝██║██║  ██║╚██████╔╝██║  ██║██║  ██║██║ ╚═╝ ██║
╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝       ╚═════╝ ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝
                                                                                                    


```
<h1 align="center">
<br>
Mono-Diagram
<br>
</h1>

<p align="center">
<a href="https://crates.io/crates/mono-diagram"><img alt="crates.io" src="https://img.shields.io/crates/v/mono-diagram.svg"></a>
<a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
</p>

<div align="center">

[English](./README.md) / [简体中文](./README-zh-CN.md) 

</div>

mono-diagram 是一个跨平台的生成纯文本图表的工具。纯文本图表的优势在于可以在任何地方显示，比如用在代码注释中。

## 使用方法

### 定义图表

使用 mono-diagram 的基本思路是写一个包含一个或多个图表定义的文件，然后传入程序，程序会生成图表。每个图表开头需要用标签声明图标类型。

<details>
<summary> 二叉树 </summary>

标签: `[binary_tree]`

输入: 

```
[binary_tree] {style: ascii}  // 声明标签
a->b,c          // 节点a有左子树b和右子树c
b->d,f          // 节点名字类似于变量名
f->fa,fb
c->k,m
k->e,           // k只有左子树
m->,x

a:2             // 给每个节点名赋值
b:0.42
c:9.5
f:-3
k:abc
m:2             // 不同的节点可以有相同的值
d:001
fa:451
fb:8.90
x:1.2
```

花括号中的键值对用于设置图表属性，此处将`style`设置为`ascii`， 你也可以设置为`unicode`。如果不设置，将会使用默认值。

输出 (ascii):

```
            ___2___
        ___/       \___
     0.42             9.5
    _/   \_         _/   \_
  001     -3      abc      2
          / \     /         \
        451 8.90  e          1.2
```

输出 (unicode):

```
               2
       ┌───────┴───────┐
     0.42             9.5
   ┌───┴───┐       ┌───┴───┐
  001     -3      abc      2
         ┌─┴─┐   ┌─┘       └─┐
        451 8.9  e          1.2
```

*注: 二叉树最底层节点只能最多容纳三个字符，对于更复杂的节点值建议使用有向非循环图*

</details>

<details>
<summary> 有向非循环图 (DAG) </summary>

标签: `[dag]`

输入:

```
[dag]
a->b    // 节点名->节点名 代表一条边
a->c    // 有向非循环图不能含有循环
b->d
c->f
c->g
a->f
d->da
d->db
g->gg
a->gg


a:Home Page     // 赋值
b:Main Section 1
c:Main Section 2
d:Subsection 1
f:Subsection 2
g:Subsection 3
da:Sub-sub
db:Sub-sub
gg:#page#
```

输出 (只有unicode版本): 

```
 ┌───────────────────────────────────────────────────┐
 │ Home Page                                         │
 └┬─────────────────┬──┬────────────────────────────┬┘
 ┌V───────────────┐ │ ┌V───────────────┐            │
 │ Main Section 1 │ │ │ Main Section 2 │            │
 └┬───────────────┘ │ └┬────────────┬──┘            │
 ┌V─────────────┐ ┌─V──V─────────┐ ┌V─────────────┐ │
 │ Subsection 1 │ │ Subsection 2 │ │ Subsection 3 │ │
 └┬───────────┬─┘ └──────────────┘ └┬─────────────┘ │
 ┌V────────┐ ┌V────────┐ ┌──────────V───────────────V┐
 │ Sub-sub │ │ Sub-sub │ │ #page#                    │
 └─────────┘ └─────────┘ └───────────────────────────┘

```

*注：此生成不稳定，每次生成可能会得到不同的图*

</details>

</details>

<details>
<summary> 表格 </summary>

标签: `[table]`

输入:

```
[table]     // 每列用 '|' 分隔，每行用换行分隔
Base Class Member|Public Inheritance|Protected Inheritance|Private Inheritance
Public|Public|Protected|Private
Protected|Protected|Protected|Private
Private|Hidden|Hidden|Hidden
```

输出 (ascii): 

```
+-------------------+--------------------+-----------------------+---------------------+
| Base Class Member | Public Inheritance | Protected Inheritance | Private Inheritance |
+-------------------+--------------------+-----------------------+---------------------+
| Public            | Public             | Protected             | Private             |
+-------------------+--------------------+-----------------------+---------------------+
| Protected         | Protected          | Protected             | Private             |
+-------------------+--------------------+-----------------------+---------------------+
| Private           | Hidden             | Hidden                | Hidden              |
+-------------------+--------------------+-----------------------+---------------------+
```

输出 (unicode): 

```
┌───────────────────┬────────────────────┬───────────────────────┬─────────────────────┐
│ Base Class Member │ Public Inheritance │ Protected Inheritance │ Private Inheritance │
├───────────────────┼────────────────────┼───────────────────────┼─────────────────────┤
│ Public            │ Public             │ Protected             │ Private             │
├───────────────────┼────────────────────┼───────────────────────┼─────────────────────┤
│ Protected         │ Protected          │ Protected             │ Private             │
├───────────────────┼────────────────────┼───────────────────────┼─────────────────────┤
│ Private           │ Hidden             │ Hidden                │ Hidden              │
└───────────────────┴────────────────────┴───────────────────────┴─────────────────────┘
```

</details>

</details>

<details>
<summary> 网格 </summary>

标签: `[grid]`

输入:

```
[grid]

width: 10       // 10列
height: 7       // 7行

1,1:a 
6,2:l               // 第六列第二行的节点值为 'l'
3,3:j
10,5:m
2,7:k
```

输出 (ascii): 

```
+---+---+---+---+---+---+---+---+---+---+
| a |   |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | l |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   | j |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | m |
+---+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|   | k |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
```

输出 (unicode): 

```
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │   │   │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │ l │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │ j │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │   │ m │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│   │ k │   │   │   │   │   │   │   │   │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
```

</details>

<details>
<summary> 甘特图 </summary>

Tag: `[gantt]`

输入:

```
[gantt] {style: unicode}

timeline: Week 1|Week 2|Week 3|Week 4|Week 5    // 时间轴

task 1|   0 ~ 0.6      // 每个task的起止时间
task 2| 0.9 ~ 2.3      // task 2 开始于第0.9周，结束于第2.3周
task 3| 2.0 ~ 2.8
task 4| 2.8 ~ 3.5
task 5| 3.5 ~ 5.0
```

输出 (ascii): 

```
        |  Week 1  |  Week 2  |  Week 3  |  Week 4  |  Week 5
--------+----------+----------+----------+----------+-----------
 task 1 |<=====>   .          .          .          .
 task 2 |         <===============>      .          .
 task 3 |          .          .<=======> .          .
 task 4 |          .          .        <=======>    .
 task 5 |          .          .          .     <================>
        |
```

输出 (unicode): 

```
           Week 1     Week 2     Week 3     Week 4     Week 5
────────────────────────────────────────────────────────────────
 task 1 │[━━━━━]   ·          ·          ·          ·
 task 2 │         [━━━━━━━━━━━━━━━]      ·          ·
 task 3 │          ·          ·[━━━━━━━] ·          ·
 task 4 │          ·          ·        [━━━━━━━]    ·
 task 5 │          ·          ·          ·     [━━━━━━━━━━━━━━━━]
        │
```

</details>

<details>
<summary> 时间轴 </summary>

Tag: `[timeline]`

输入:

```
[timeline] {style: unicode}

2022.06|Some things happened in 2022        // 格式为 <TIME>|<EVENT>
2023|                                       // event可以为空
2024.11|Some things that is happening now
2030.01|Some things that will happen in the future
```

输出 (ascii): 

```
    |
    |
    |
----v----
 2022.06  >--- Some things happened in 2022
----v----
    |
 ---v---
   2023
 ---v---
    |
----v----
 2024.11  >--- Some things that is happening now
----v----
    |
----v----
 2030.01  >--- Some things that will happen in the future
----v----
    |
    |
    |
    V
```

输出 (unicode): 

```
    ║
    ║
    ║
    ╨
 2022.06  ┄┄┄┄ Some things happened in 2022
    ╥
    ║
    ╨
   2023
    ╥
    ║
    ╨
 2024.11  ┄┄┄┄ Some things that is happening now
    ╥
    ║
    ╨
 2030.01  ┄┄┄┄ Some things that will happen in the future
    ╥
    ║
    ║
    ║
    ▼
```

</details>

### 属性

属性用于设置图表的风格
格式为: 

```
[<DIAGRAM TAG>] {<KEY1: VALUE1>, <KEY2: VALUE2>, ...}
...
```
所有属性:
 - `style`: `ascii` / `unicode`

目前只有一个属性，未来会添加更多

### 命令行参数

```
用法: mono-diagram [OPTIONS] <FILE_PATH>

参数:
  <FILE_PATH>
          文件路径

选项:
  -p, --prefix <PREFIX>
          给输出的每行加一个前缀

          可用于代码注释

  -c, --copy
          复制到剪贴板

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

#### 示例命令 

```bash
mono-diagram examples/test -c -p "# " 
```

## 安装

请先安装 [rust](https://www.rust-lang.org/tools/install) 

然后运行命令：

~~~bash
cargo install mono-diagram
~~~

## TODO

1. dag 中的节点可以是表格 (这样就可以画类图)
2. 优化 dag 中边的绘制
3. 增加韦恩图
4. 增加时序图
5. 增加表格的单元格合并

## 贡献和帮助

非常欢迎 contribution，就算只是 contribute idea 也很欢迎

如有 bug 请开 issue （dag 应该是有 bug 的）

