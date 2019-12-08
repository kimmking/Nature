# Concepts

The Nature System focus on **data** and **relations** between data.  Data are the  **business goals**, and `relation` can let you achieve goal one by one.  

`instance`s are the final things that you want which are generated by `relation`'s `converter` in consistent way.

This is little like workflow or BPMN,  but they are two very different things. The last tell us to **do something**, yet the first tell us we **need something**. How-to-do is very complex. and what-your-need is simple and more important.  Nature let you focus on the things which are really important and let you manage them easily.

## Data

Data are immutable, when they are created you can't modify or delete it.

There are two kind of data: `meta` and `instance`.  

[Meta](concept-meta.md)

[Instance](concept-instance.md)

## Relation

A relation will connect tow and only two `meta`s.  but one `meta` can connect to many other `meta`s by relations, so you can make a complex business web by `relation`s between `meta`s, and avoid the hurt of the complex business system.

A `relation` has direction, begin from **from**-`meta` end to **to**-`meta`.

`relation` decouple you application into pieces and **glue all your logical together at design-time**.

A `relation` defined a `converter` to convert from **from**-`meta`'s instance' to **to**-`meta`'s instance, and that will be continue until there is no `relation` defined for upstream `meta`.

`converter` will **glue all your logical together at runtime**. 

[converter](concept-converter.md)