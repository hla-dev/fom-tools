# fom check

`fom check` inspects a FOM for conformance with the HLA OMT Standard
(1516.2-2010) and other style conventions (to be determined). Think of it as a
validator and linter for HLA object models.

HLA object models capture, in XML documents, the data that can be communicated
within a federation. There are a number of criteria that need to be met to have
a "HLA conforming" object model:

1. valid XML document,
2. valid HLA OMT syntax,
3. conforming HLA OMT values.

`fom check` will check these three criteria as well as a fourth:

4. style conventions/best practices

What constitutes this fourth is not yet defined (need to check whether Brad
Dillman wrote anything on this). This tool provides an option for the HLA
community to begin documenting a style guide. Something akin to
[rust-clippy](https://rust-lang.github.io/rust-clippy/master/index.html).

The first three checks should be relatively easy.

1. An XML document can be checked for validity by simply parsing it,
2. An object model can be checked for validity using the OMT DIF Schema (see
   Annex D of 1516.2-2010),
3. An object model can be checked for conformance using the OMT conformance XML
   Schema (see Annex E of 1516.2-2010). This contains the DIF Schema.

The style conventions will be checked with custom functions.
