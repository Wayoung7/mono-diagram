<!-- README-en.md -->


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

[English](./README.md) | [简体中文](./README-zh-CN.md) 

</div>

A cross-platform command line tool for generating plain-text diagrams from a certain syntax. The biggest advantage of plain-text diagrams is that it can fit in anywhere.

## Usage

### Define diagrams

 The basic idea of mono-diagram is to define diagram in a file, and pass it to the program to generate planar diagrams. A file can contain multiple diagrams. Each diagram must start with a tag such as `[table]` to tell the program what kind of diagram it is.

<details>
<summary> Binary Tree </summary>

Tag: `[binary_tree]`

Input file: 

```
[binary_tree] {style: ascii}  // Specify diagram category
a->b,c          // Node 'a' has left child 'b' and right child 'c'
b->d,f          // Node name is just like variables
f->fa,fb
c->k,m
k->e,           // Node 'k' only has one left child
m->,x

a:2             // Assign values to node
b:0.42
c:9.5
f:-3
k:abc
m:2             // Different nodes can have same value
d:001
fa:451
fb:8.9
x:1.2
```

The key-value pair in curly brakets gives attributes to the diagram, every attribute has a default value if you didn't specify. We set `style` to `ascii`, but you can also set it to `unicode` to change the appearance of the diagram.

Output diagram (ascii):

```
            ___2___
        ___/       \___
     0.42             9.5
    _/   \_         _/   \_
  001     -3      abc      2
          / \     /         \
        451 8.9  e          1.2
```

Output diagram (unicode):

```
               2
       ┌───────┴───────┐
     0.42             9.5
   ┌───┴───┐       ┌───┴───┐
  001     -3      abc      2
         ┌─┴─┐   ┌─┘       └─┐
        451 8.9  e          1.2
```

Use `binary_tree` only if the node value is very simple, because nodes on the bottom line of binary tree can only hold up to 3 characters. For node with complicated value, use `dag` instead.

</details>

<details>
<summary> Directed Acyclic Graph (DAG) </summary>

Tag: `[dag]`

Input file:

```
[dag]
a->b    // <NODE-NAME>-><NODE-NAME> represents an edge in the graph
a->c    // The graph cannot have cycles
b->d
c->f
c->g
a->f
d->da
d->db
g->gg
a->gg


a:Home Page     // Assign values
b:Main Section 1
c:Main Section 2
d:Subsection 1
f:Subsection 2
g:Subsection 3
da:Sub-sub
db:Sub-sub
gg:#page#
```

Output diagram (dag only has unicode version): 

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

*Note: this dag graph is not stable, meaning you may get graph with different looking each time*

</details>

</details>

<details>
<summary> Table </summary>

Tag: `[table]`

Input file:

```
[table]     // Each column is seperated by '|' and each row is seperated by newline
Base Class Member|Public Inheritance|Protected Inheritance|Private Inheritance
Public|Public|Protected|Private
Protected|Protected|Protected|Private
Private|Hidden|Hidden|Hidden
```

Output diagram (ascii): 

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

Output diagram (unicode): 

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
<summary> Grid </summary>

Tag: `[grid]`

Input file:

```
[grid]

width: 10       // The grid has 10 colums
height: 7       // And 7 rows

1,1:a 
6,2:l           // The cell at column 6, row 2 has content 'l'
3,3:j
10,5:m
2,7:k
```

Output diagram(ascii): 

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

Output diagram(unicode): 

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
<summary> Gantt Diagram </summary>

Tag: `[gantt]`

Input file:

```
[gantt] {style: unicode}

timeline: Week 1|Week 2|Week 3|Week 4|Week 5    // Specify the time line

task 1|   0 ~ 0.6      // Specify the time period of each task
task 2| 0.9 ~ 2.3      // This means task 2 starts at Week 0.9 and ends at Week 2.3 (you know what I mean)
task 3| 2.0 ~ 2.8
task 4| 2.8 ~ 3.5
task 5| 3.5 ~ 5.0
```

Output diagram(ascii): 

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

Output diagram(unicode): 

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
<summary> Timeline </summary>

Tag: `[timeline]`

Input file:

```
[timeline] {style: unicode}

2022.06|Some things happened in 2022        // The format is <TIME>|<EVENT>
2023|                                       // Time can have no event
2024.11|Some things that is happening now
2030.01|Some things that will happen in the future
```

Output diagram(ascii): 

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

Output diagram(unicode): 

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

### Attributes

Attributes are used to give diagram styled looking.
The format is: 

```
[<DIAGRAM TAG>] {<KEY1: VALUE1>, <KEY2: VALUE2>, ...}
...
```
All attributs:
 - `style`: `ascii` / `unicode`

Currently, `Attrib` only contain `style`, but more will be added in the future.

### Command Line Arguments

```
Usage: mono-diagram [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>
          The path to the input file

Options:
  -p, --prefix <PREFIX>
          Add a prefix to each line in the output

          This is useful when you want to paste the diagram to code comments

  -c, --copy
          Copy the output to your computer clipboard

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

#### Example commands

Parse the file examples/test, and output with prefix '# ', then copy to clipboard: 

```bash
mono-diagram examples/test -c -p "# " 
```

## Examples

You can find some sample input files in [`examples/`](./examples/) in the project directory.

## Installation

Please first install [rust](https://www.rust-lang.org/tools/install).

Then, simply run the following commands:

~~~bash
cargo install mono-diagram
~~~

## TODO

1. Node in dag can be a table. This enables you to draw class diagram 
2. Improve edge drawing in dag 
3. Add plain-text Venn diagram 
4. Add plain-text sequence diagram 
5. Make more flexible table with cells spanning multiple rows and columns

## Contribution and Help

Any contribution is welcome, even just ideas.

Feel free to open an issue or contact me if you find any bugs.

