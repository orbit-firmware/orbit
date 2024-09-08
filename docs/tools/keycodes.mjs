// generates markdown files for all keycodes

import fs from 'fs';
import path from 'path';

const dir = '../rmk/keycodes';
const files = fs.readdirSync('../rmk/keycodes').filter(file => file.endsWith('.k'));



let available = [];

for (const file of files) {
  let name = file.replace('.k', '');
  name = name.replace('_', ' ');
  name = name.trim();

  let current_section = "";
  let content = {}
  fs.readFileSync(path.join(dir, file), 'utf-8').split('\n').forEach(line => {
    line = line.trim();
    if (line === "") return;
    if (line.startsWith("###")) {
      current_section = line.replace('###', '').trim();
      return;
    };
    if (line.startsWith("#")) return;
    if (line.indexOf('->') === -1) return;
    let parts = line.split('->');

    let alias_list = [
      "`" + parts[0].trim().toLowerCase() + "`"
    ];

    for (let i = 0; i < parts.length; i++) {
      parts[i] = parts[i].trim();
      if (i === 2) {
        parts[i] = parts[i].replace("\\\\", "%%BS%%");
        parts[i] = parts[i].replace("\\,", "%%COMMA%%");

        let a = parts[i].split(',');
        for (let j = 0; j < a.length; j++) {
          a[j] = a[j].trim();
          a[j] = a[j].replace("%%BS%%", "\\");
          a[j] = a[j].replace("%%COMMA%%", ",");
          if (a[j] === "`") {
            alias_list.push("`` ` ``");
          } else {
            alias_list.push("`" + a[j].trim() + "`");
          }
        }

      }
    }
    parts[2] = alias_list.join(', ');


    content[current_section] = content[current_section] || [];
    content[current_section].push(parts);
  });


  if (Object.keys(content).length === 0) continue;

  let markdown = '# ' + name;

  if (name === 'us') {
    markdown += ' (Default)';
  }
  markdown += '\n\n';

  for (const section in content) {
    markdown += `## ${section}\n\n`;
    markdown += '| Key | Code | Alias |\n';
    markdown += '| --- | --- | --- |\n';
    for (const line of content[section]) {
      if (line.length < 3) {
        markdown += `| ${line[0]} | ${line[1]} |  |\n`;
      } else {
        markdown += `| ${line[0]} | ${line[1]} | ${line[2]} |\n`;
      }

    }
    markdown += '\n';
  }

  if (!fs.existsSync('keycodes')) {
    fs.mkdirSync('keycodes');
  }
  fs.writeFileSync(path.join('keycodes', name + '.md'), markdown);
  available.push({ text: name, link: '/keycodes/' + name });
}

let template = `export default [
  ${available.map(item => `{ text: '${item.text}', link: '${item.link}' },`).join('\n  ')}
];`;

fs.writeFileSync('.vitepress/keycodes.mts', template);