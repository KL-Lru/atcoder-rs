# 競プロ日誌

AtCoder をひたすらに Rust で解いたコードを載せるだけのサイトです.

## AtCoder Beginner Contest

<div class="abc">
{%- set problems = ["a", "b", "c", "d", "e", "f", "g", "h"] -%}

<table>
  <thead>
    <tr>
      <th>Contest</th>
      {%- for problem in problems %}
      <th>{{ problem | upper }}</th>
      {%- endfor %}
    </tr>
  </thead>

  <tbody>
  {%- for contest in beginner_contests -%}
    {%- set url_head = "abc_" ~ contest.contest_number -%}
    {%- set solved = contest.solved_tasks %}
    <tr class="{% if loop.index % 2 == 1 %}odd{% else %}even{% endif %}">
      <th>
        <a href="{{ url_head }}/"> ABC {{ contest.contest_number }} </a>
      </th>
      {%- for problem in problems %}
        <td class="{% if loop.index <= 4 %}before_half{% else %}after_half{% endif %}"> {%- if problem in solved -%} <a href="{{ url_head }}/{{ problem }}/">{{ problem | upper }}</a> {%- endif -%} </td>
      {%- endfor %}
    </tr>
    {%- endfor %}
  </tbody>
</div>
