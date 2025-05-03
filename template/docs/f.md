{%- assign contest-name = project-name | replace: '_', '' -%}
{%- assign contest-type = project-name | split: '_' | first -%}
{%- assign contest-number = project-name | split: '_' | last -%}

## {{ contest-type | upcase }} {{ contest-number }} F -

refs: <https://atcoder.jp/contests/{{ contest-name | downcase }}/tasks/{{ contest-name | downcase }}_f>
