dft.p
--------------------------------------------------------------------------------
dft instructions file parser


description
--------------------------------------------------------------------------------
this lib/bin project is the official tool to parse, lint and fix .dft files


why
--------------------------------------------------------------------------------
while working in the dft project I realized 2 things:

  - the dft tool itself shouldn't be responsible of parsing the instructions
    file, but rather the execution itself

  - haven't made a parser yet, so... here's one


grammar (BNF)
--------------------------------------------------------------------------------
<program>                     :=  <statement>
                              |   <program> <statement>
                              ;

<statement>                   :=  <directive-component>
                                    <connector-component-list> ;
                              ;

<directive-component>         :=  <directive-literal> <field-list>
                              ;

<directive-literal>           :=  SET
                              |   ADD
                              |   ALIAS
                              |   MERGE
                              |   IGNORE
                              |   RENAME
                              |   FILTER
                              |   COERCE
                              |   DISTINCT
                              |   VALIDATE
                              ;

<field-list>                  :=  <field>
                              |   <field-list> , <field>
                              ;

<field>                       := String
                              ;

<connector-component-list>    :=  <connector-component>
                              |   <connector-component-list>
                                    <connector-component>
                              ;

<connector-component>         :=  <connector-literal> <target-component>
                              ;

<connector-literal>           :=  OR
                              |   TO
                              |   TYPED
                              |   RESCUE
                              |   DEFAULT
                              |   MATCHING
                              ;

<target-component>            :=  <data-value>
                              |   <data-type-literal>
                              |   <format-literal>
                              |   <action-literal>
                              |   <expression-literal>
                              ;

<data-value>                  :=  f64
                              |   String
                              |   bool
                              |   isize
                              ;

<data-type-literal>           :=  FLOAT
                              |   STRING
                              |   INTEGER
                              |   BOOLEAN
                              ;

<format-literal>              :=  URI
                              |   UUID
                              |   DATE
                              |   TIME
                              |   EMAIL
                              |   DATETIME
                              ;

<action-literal>              :=  HALT
                              |   NOTIFY
                              |   DISCARD
                              ;

<expression-literal>          :=  EQUALS
                              |   LESSER
                              |   DIFFERS
                              |   GREATER
                              |   EQLESSER
                              |   EQGREATER
                              ;


TODO
--------------------------------------------------------------------------------
[] Improve combinators that return lists
[] Improve error verbosity
