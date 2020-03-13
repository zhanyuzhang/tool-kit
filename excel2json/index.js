const $ = id => document.getElementById(id);
const $textarea = $('textarea');
const $btnParse = $('btnParse');
const $division = $('division');
const $type = $('type');

$btnParse.onclick = function() {
  const input = $textarea.value;
  const division = $division.value || '\t';
  const type = $type.value || 'object';

  try {
    // 如果已经是合法的 JSON 了则不需要再转换了
    JSON.parse(input);
  } catch (err) {
    // 以换行符号 \n 为分割符，把数据转成数组项
    const rows = input
      .split('\n')
      .filter(row => row && row.trim())
      .map(row => row.split(division));
    let output = null;

    // 如果是按行生成数组，则直接返回即可
    if (type === 'array') {
      output = rows;

      // 如果是按行生成对象，则需要把第一行取出来作为键值，然后把后面所有行生成转换成对象
    } else {
      const keys = rows.shift();
      output = rows.map(row =>
        row.reduce((acc, cur, i) => {
          acc[keys[i]] = cur;
          return acc;
        }, {})
      );
    }

    $textarea.value = JSON.stringify(output, null, 2);
  }
};
