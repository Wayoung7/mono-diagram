```
                                      _ _                                 
                                     | (_)                                
 _ __ ___   ___  _ __   ___ ______ __| |_  __ _  __ _ _ __ __ _ _ __ ___  
| '_ ` _ \ / _ \| '_ \ / _ \______/ _` | |/ _` |/ _` | '__/ _` | '_ ` _ \ 
| | | | | | (_) | | | | (_) |    | (_| | | (_| | (_| | | | (_| | | | | | |
|_| |_| |_|\___/|_| |_|\___/      \__,_|_|\__,_|\__, |_|  \__,_|_| |_| |_|
                                                 __/ |                    
                                                |___/                     
```
<h1 align="center">
<br>
Mono-Diagram
<br>
</h1>

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

</details>

</details>

<details>
<summary> Table </summary>

Tag: `[table]`

Input file:

```
[table]     \\ Each column is seperated by '|' and each row is seperated by newline
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

### Pass to program (Command Line Arguments)

Binary name: `mono-diagram`

## Installation

Install [rust](https://www.rust-lang.org/tools/install) if you havn't.

Then, simply run the following commands:

~~~bash
cargo install mono-diagram
~~~


