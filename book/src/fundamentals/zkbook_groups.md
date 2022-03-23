# Groups

Groups are an algebraic structure which

- are used in most widely deployed signature schemes

- are used in most widely deployed zk-SNARKs

- generalize the notion of a collection of symmetries of an object

- generalize certain aspects of numbers

I will not touch on the last two too much.

First let's do a quick definition and then some examples.

**Definition.** A group is a set $G$ with a binary operation $* \colon G \times G \to G$, an identity $e \in G$ for that operation, and a inverse map $(\quad)^{-1} \colon G \to G$ such that

- $*$ is associative: $x*(y*z) = (x*y)*z$ for all $x, y,z$.

- $e$ is an identity for $*$. I.e., $e * x = x * e = x$ for all $x$.

- $(\quad)^{-1}$ is an inverse map for $*$ with identity $e$. I.e.,  $x * x^{-1} = x^{-1} * x = e$ for all $x$.

So basically, an invertible binary operation. Definition in hand, we can see some examples:

- The integers $\mathbb{Z}$ with $+$, $0$ as identity, and negation as inverse.

- For any natural number $n$, the integers mod $n$. This can be thought of as the set of numbers $\{0, \dots, n - 1\}$ with the operation being $+$ followed by taking remainder mod $n$. It can also be thought of as the group of rotations of an $n$-gon.

- For any set $X$, the set of invertible functions $X \to X$ (a.k.a permutations) with function composition as the operation, the identity function as the identity, and function inverse as the inverse operation.

- The set of translations of a plane $\mathbb{R} \times \mathbb{R}$ with composition of translations (since the composition of translations is a translation). The identity is translating by $(0, 0)$ and inverse is reversing the translation. This is equivalent to the group $\mathbb{R} \times \mathbb{R}$ with coordinate-wise addition as the operation and coordinate-wise negation as the inverse.

- The set of rotations of a sphere with composition as the operation, doing nothing as the identity, and performing the rotation in reverse as the inverse map.

- For any field $F$, the set $F^{\times} := F \setminus \{0\}$, with field multiplication as the operation, $1$ as the identity, and $x \mapsto 1 / x$ as the inverse map. This is called the **group of units** of $F$.

Sometimes, instead of $x * y$ we simply write $xy$ when $*$ is clear from context.

### Abelian groups

An abelian group is a group in which $*$ is commutative, meaning $x * y = y * x$. Typically, in that case, we write the group operation as $+$ instead and we write the identity as $0$.

Some examples are

- the integers with addition

- the integers mod $n$ with addition

- the group of units of a field $F$

- the real numbers with addition

- the rational numbers with addition

- the real numbers with multiplication

- the rational numbers with multiplication

- vectors of integers of some fixed dimension with coordinate-wise addition

- The set of polynomials over a ring $R$, $R[x]$ with addition

Abelian groups are equivalently described as what are called $\Z$-modules. A $\Z$-module is like a vector space, but with the integers instead of a field.

Basically a $\mathbb{Z}$-module (or equivalently an Abelian group) is a structure where you can add elements and where it makes sense to multiply a group element by an integer.

If $G$ is an abelian group, we can define this "multiplication by an integer" as follows. If $n \geq 0$, then for $g \in G$, we can define $n \cdot g$ by

$$
n \cdot g := \underbrace{g + \dots + g}_{n}
$$

and if $n < 0$, then define $n \cdot g := |n| \cdot (-g)$. Or equivalently.,

$$
n \cdot g = \underbrace{(-g) + \dots + (-g)}_{|n|} = -\left( \underbrace{g + \dots + g}_{|n|} \right)
$$

This is the general sense of what is called **scalar-multiplication** or sometimes **exponentiation** in cryptography.

### Cyclic groups

A cyclic group $G$ is a special kind of abelian group. It is an abelian group generated by a single element $g \in G$. That is, a cyclic group $G$ (generated by $g \in G$) is one in which for every $h \in G$  we have $h = n g$ for some $n \in \mathbb{Z}$.

### Groups in cryptography

Many cryptographic protocols are defined generically with respect to any abelian group that has a "hard discrete log". 

Let 

- $G$ be a cyclic group

- $g$ a generator of $G$

- $\mu$ a probability distribution on $G$

- $\mathcal{A}$ a set of algorithms of type $G \to \mathbb{Z}$ with runtime and memory usage bounds. In other words, a set of tuples $(P, t, m)$ where $P$ is a program of type $G \to \Z$, $t$ is a bound on the time that program can be run, and $m$ is a bound on the amount of memory that program can use.
  
  In practice you fix this to be something like, the set of all computations that can be run for less than 1 trillion \$.

- $\varepsilon \in [0, 1]$ a probability, usually taken to be something close to 0 like $1 / 2^{128}$ 

Then we can define the **$(G, g, \mu, \mathcal{A}, \varepsilon)$-computational discrete-log assumption** which says:

> For any $(P, t, m) \in \mathcal{A}$, if you sample h from G according to $\mu$, then the probability that $P(h) \cdot h = g$ is smaller than $\varepsilon$, assuming $P$ successfully runs within the given resource bounds.

Sometimes this is called the computational Diffie--Helman assumption. Basically what it's saying is that for a group element $h$ sampled randomly, you can't practically compute how many times you have to add $g$ to itself to get $h$.

Another really important assumption is the no-relation assumption (TODO: name).

Basically what this says is that randomly selected group elements have no efficiently computable linear relations between them. Formally, letting $G$ be a group and $\mu$ a probability distribution on $G$, and $\mathcal{A}$ a set of programs (with resource bounds) that take in a list of group elements as inputs and outputs a list of integers of the same length.

Then the $(G, \mu, \mathcal{A}, N,\varepsilon)$-no-relation assumption says for all $(P,t,m) \in \mathcal{A}$, for any $n \leq N$, if you sample $g_1, \dots, g_n$ according to $G$, letting $[e_1, \dots, e_n] = P([g_1, \dots, g_n])$, it is not the case that

$$
e_1 \cdot g_1 + \dots + e_n \cdot g_n = 0
$$

except with probability $\varepsilon$ (assuming program $P$ runs in time $t$ with memory $m$).

#### Generic group model

### Elliptic curves

Now, what are some concrete groups that we can safely make the no-relation or computational discrete log hardness assumptions about?

Well, the most efficiently implementable groups that people have come up with -- and that we believe satisfy the above assumptions for $\mathcal{A}$ being the class of realistic computations and $\varepsilon$ being something like $1/2^{128}$ -- are elliptic curves over finite fields.

Giving a complete definition of what  an elliptic curve is requires a lot of math, and is not very useful from the point of view of cryptography. So we will give a definition that is not complete, but more useful.

An elliptic curve $E$ over a field $F$ is a set of the form

$$
\{ (x, y) \in F \times F \mid y^2 = x^3 + ax + b \}
$$

for some $a, b \in F$, plus an additonal point $O$ which is called the point at infinity. Basically it's a set of pairs satisfying some equation of that form. The data of the elliptic curve itself is just the field $F$ together with the constants $a$ and $b$. 

What is special about elliptic curves is that there is a way of giving them a group structure with simple to compute operations that we believe satisfy the hardness assumptions described above.

Group negation $(-) \colon E \to E$ is defined by

$$
-(x_0, y_0) = (x_0, - y_0)
$$

so we just negate the $y$ coordinate.

The identity for the group is $O$, the point at infinity. For that point we may also therefore write it as $0$.

Group addition is more complicated to define, so we will not, but here is what's worth knowing about how to compute $(x_0, y_0) + (x_1, y_1)$

- There are three cases
  
  1. $x_0 \neq x_1$
  
  2. $x_0 = x_1$ and $y_0 = y_1$
  
  3. $x_0 = x_1$ but $y_0 \neq y_1$. In this case it turns out $y_0 = -y_1$ and so the two points are inverse, and we return $O$ 
  
  In cases 1 and 2, the algorithm to compute the result just performs a simple sequence of some field operations (multiplications, additions, divisions, subtractions) with the input values. In other words, there is a simple arithmetic formula for computing the result.

#### Elliptic curves in Rust

#### Elliptic curves in Sage

#### Serializing curve points

Given a curve point $(x,y)$ we know $y^2 = x^3 + ax + b$ and thus $y$ is one of the two square roots of $x^3 + ax + b$. If $y$ is a given square root, the other square root is $-y$ since $y^2 = (-y)^2$. So, if we know $x$, then we almost know the whole curve point: we just need a single bit to tell us which of the two possible values (i.e., the two square roots of $x^3+ax+b$) is the y coordinate.

Here is how this is commonly done over prime order fields $\mathbb{F}_p$, assuming $p \neq 2$. Remember that we represent elements of $\mathbb{F}_p$ as integers in the range $[0, p-1]$. In this representation, for a field element $y$, $-y$ is the integer $p - y$ (or $0$ if $y = 0$). Thus, if $y$ is odd, then $-y$ is even (since $p$ is odd and an odd minus an odd is even). Similarly, if $y$ is even, then $- y$ is odd (since an odd minus an even is odd).

So, for any $y$, unless $y$ is 0, $y$ and $-y$ have opposite parities. Parity can easily be computed: it's just the least significant bit. Thus, we have an easy way of encoding a curve point $(x, y)$. Send

- $x$ in its entirety

- The least significant bit $b_0$ of $y$

Given this, we can reconstruct $y$ as follows. 

1. Compute any square root $Y$ of $x^3 + ax + b$ 

2. If the least significant bit of $Y$ is equal to $b_0$, then $y = Y$, otherwise, $y = -Y$

In the case of Mina, our field elements require 255 bits to store. Thus, we can encode a curve point in $255 + 1 = 256$ bits, or 32 bytes. At the moment this optimized encoding is not implemented for messages on the network. It would be a welcome improvement.

#### Projective form / projective coordinates

The above form of representing curve points as pairs -- which is called **affine form** -- is sometimes sub-optimal from an efficiency perspective. There are several other forms in which curve points can be represented, which are called projective forms.

The simple projective form represents a curve $E$ as above as the set

$$
\{ (X, Y, Z) \in F^3 \mid (Y/Z)^2 = (X/Z)^3 + a(X/Z) + b \}
$$

If you think about it, this is saying that $(X/Z, Y/Z)$ is a point on the original curve in affine form. In other words,  in projective form we let the first two coordinates get scaled by some arbitrary scaling factor $Z$, but we keep track of it as the third coordinate.

To be clear, this means curve points have many different representations. If $(x, y, z)$ is a curve point in projective coordinates, and $s$ is any element of $F$, then $(sx,sy,sz)$  is another representation of the same curve point.

This means curve points require more space to store, but it makes the group operation much more efficient to compute, as we can avoid having to do any field divisions, which are expensive. 

##### Jacobian form / Jacobian coordinates

There is another form, which is also sometimes called a projective form, which is known as the jacobian form. There, a curve $E$ would be represented as

$$
\{ (X, Y, Z) \in F^3 \mid (X / Z^2, Y / Z^3) \in E \}
$$

so the triple $(X, Y, Z)$ corresponds to the affine point $(X/Z^2, Y/Z^3)$. These are the fastest for computing group addition on a normal computer.

#### Take aways

- Use affine coordinates when the cost of division doesn't matther and saving space is important. Specific contexts to keep in mind:
  
  - Working with elliptic curve points inside a SNARK circuit

- Use Jacobian coordinates for computations on normal computers, or other circumstances where space usage is not that costly and division is expensive.

- Check out this [website](https://www.hyperelliptic.org/EFD/) for the formulas for implementing the group operation.

### More things to know

- When cryptographers talk about "groups", they usually mean a "computational group", which is a group equipped with efficient algorithms for computing the group operation and inversion. This is different because a group in the mathematical sense need not have its operations be computable at all.

## Exercises

- Implement a type `JacobianPoint<F:Field>` and functions for computing the group operation

- Familiarize yourself with the types and traits in `ark_ec`. Specifically,
  
  - todo

- Implement `fn decompress<F: SquareRootField>(c: (F, bool)) -> (F, F)`

- 