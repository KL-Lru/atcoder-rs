{%- assign contest-name = project-name | replace: '_', '' -%}
{%- assign contest-type = project-name | split: '_' | first -%}
{%- assign contest-number = project-name | split: '_' | last -%}

[Back to Top](/atcoder-rs/)

## {{ contest-type | upcase }} {{ contest-number }}

<https://atcoder.jp/contests/{{ contest-name | downcase }}>
