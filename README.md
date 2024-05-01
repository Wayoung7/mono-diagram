<!-- README-en.md -->


```

 __    __     ______     __   __     ______     _____     __     ______     ______     ______     ______     __    __    
/\ "-./  \   /\  __ \   /\ "-.\ \   /\  __ \   /\  __-.  /\ \   /\  __ \   /\  ___\   /\  == \   /\  __ \   /\ "-./  \   
\ \ \-./\ \  \ \ \/\ \  \ \ \-.  \  \ \ \/\ \  \ \ \/\ \ \ \ \  \ \  __ \  \ \ \__ \  \ \  __<   \ \  __ \  \ \ \-./\ \  
 \ \_\ \ \_\  \ \_____\  \ \_\\"\_\  \ \_____\  \ \____-  \ \_\  \ \_\ \_\  \ \_____\  \ \_\ \_\  \ \_\ \_\  \ \_\ \ \_\ 
  \/_/  \/_/   \/_____/   \/_/ \/_/   \/_____/   \/____/   \/_/   \/_/\/_/   \/_____/   \/_/ /_/   \/_/\/_/   \/_/  \/_/ 
                                                                                                                         

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

A cross-platform command line tool for generating plain-text diagrams from a certain syntax. The biggest advantage of plain-text diagrams is that it can fit in anywhere.

## Usage

### Define diagrams

 The basic idea of mono-diagram is to define diagram in a file, and pass it to the program to generate planar diagrams. A file can contain multiple diagrams. Each diagram must start with a tag such as `[table]` to tell the program what kind of diagram it is.

<details>
<summary> Binary Tree </summary>

Tag: `[binary_tree]`

Input file: 

```
[binary_tree]   // Specify diagram category
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
fb:8.90
x:1.2
```

Output diagram:

```
            ___2___
        ___/       \___
     0.42             9.5
    _/   \_         _/   \_
  001     -3      abc      2
          / \     /         \
        451 8.90  e          1.2
```

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

Output diagram: 

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

*Note: this dag graph is not stable, meaning you may get graph with different looking*

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

Output diagram: 

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

</details>

</details>

<details>
<summary> Grid </summary>

Tag: `[grid]`

Input file:

```
[grid] {10, 7}      // The grid has 10 colums and 7 rows

1,1:a 
6,2:l               // The cell at column 6, row 2 has content 'l'
3,3:j
10,5:m
2,7:k
```

Output diagram: 

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

</details>

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

Install [rust](https://www.rust-lang.org/tools/install) if you havn't.

Then, simply run the following commands:

~~~bash
cargo install mono-diagram
~~~

## TODO

1. Node in dag can be a table. This enables you to draw class diagram (this is easy)
2. Improve edge drawing in dag (this is hard)
3. Add plain-text Venn diagram (this is hard)
4. Add plain-text sequence diagram (this is easy)
5. Make more flexible table with cells spanning multiple rows and columns

## Contribution and Help

Any contribution is always welcomed, even just ideas.

Feel free to open an issue or contact me if you find any bugs.

