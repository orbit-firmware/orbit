import fs from 'fs';
import path from 'path';
import modifiers from './modifiers.mjs';

const DIR = '../orbit/keycodes';
const FILES = fs.readdirSync(DIR).filter(file => file.endsWith('.kcs'));
const BASE_FILE = 'us.kcs';

function convert_name(name) {
  name = name.replace("_", " ");
  name = name.split(" ").map((word, index) => {
    if (word.length == 2) {
      return word.toUpperCase();
    }
    return word.charAt(0).toUpperCase() + word.slice(1);
  })

  if (name.length == 1) {
    return name[0];
  }

  return name[0] + " (" + name.slice(1).join(" ") + ")";
}

// list of keycode files
let keycode_files = [
  { text: convert_name("us"), link: '/keycodes/' + "us" }
];
let base_keycodes = {};
let us_pushed = false;

const merge_keycodes = (base, adjustments) => {
  let merged = JSON.parse(JSON.stringify(base));

  for (const section in adjustments) {
    merged[section] = merged[section] || [];

    adjustments[section].forEach(adjustment => {
      const key = adjustment[0];

      const existing_index = merged[section].findIndex(item => item[0] === key);

      if (existing_index !== -1) {
        const existing_item = merged[section].find(item => item[0] === key);
        let alias = [...new Set([...existing_item[2], ...adjustment[2]])];
        adjustment[2] = alias;
        merged[section][existing_index] = adjustment;
        merged[section][existing_index].adjusted = true;
      } else {
        adjustment.adjusted = true;
        merged[section].push(adjustment);
      }
    });
  }

  return merged;
}

const parse_keycode = (code) => {
  if (code.startsWith("0x")) {
    return code;
  }

  code = code.replace(")", "");
  let tokens = code.split("(").reverse();

  if (!tokens[0].startsWith("0x")) {
    for (const section in base_keycodes) {
      base_keycodes[section].forEach(keycode => {
        if (keycode[0] === tokens[0]) {
          tokens[0] = parse_keycode(keycode[1]);
        }
      });
    }
  }

  code = parseInt(tokens[0], 16);
  tokens.shift();
  for (const i in tokens) {
    let mod = tokens[i];
    code = modifiers[mod](code);
  }

  code = '0x' + code.toString(16).padStart(4, '0').toUpperCase();
  return code;
}

const parse_file = (file_path) => {
  let content = {};
  let current_section = "";

  fs.readFileSync(file_path, 'utf-8').split('\n').forEach(line => {
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
          if (a[j] === "|") {
            alias_list.push("`\\|`");
          } else if (a[j] === "`") {
            alias_list.push("`` ` ``");
          } else {
            alias_list.push("`" + a[j].trim() + "`");
          }
        }
      }
    }
    parts[2] = alias_list;

    content[current_section] = content[current_section] || [];
    content[current_section].push(parts);
  });

  return content;
}


base_keycodes = parse_file(path.join(DIR, BASE_FILE));


for (const file of FILES) {
  const name = file.replace('.kcs', '').replace('_', ' ').trim();
  const file_path = path.join(DIR, file);

  const adjustments = parse_file(file_path);
  if (Object.keys(adjustments).length === 0) continue;

  const merged = merge_keycodes(base_keycodes, adjustments);

  let markdown = '# ' + convert_name(name);

  if (name === 'us') {
    markdown += ' (Default)';
  }
  markdown += '\n\n';



  markdown += '## Usage\n\n';
  if (name === 'us') {
    markdown += '::: info\n';
    markdown += 'Optional, since this is the default.  \n';
    markdown += ':::\n';
  }

  markdown += '```toml\n';
  markdown += `# keyboard.toml\n`;
  markdown += `[settings]\n`;
  markdown += `keycodes = "${name}" // [!code focus]\n`;
  markdown += '```\n\n';

  if (name !== 'us') {
    markdown += '#### Legend\n';
    markdown += '<span style="color: var(--vp-c-brand-1)">`*`</span> Keys have been adjusted to meet the language specific layout.  \n';
  }

  for (const section in merged) {
    markdown += `## ${section}\n\n`;
    markdown += '| Key | Code | Alias |\n';
    markdown += '| --- | --- | --- |\n';

    for (const line of merged[section]) {
      let ident = line[0];
      if (line.adjusted) {
        ident += "<a style='color: var(--vp-c-brand-1); text-decoration: none;' href='#legend'>*</a>";
      }
      let code = parse_keycode(line[1]);
      let alias = line.length < 3 ? "" : line[2].join(', ');
      markdown += `| ${ident} | ${code} | ${alias} |\n`;
    }
    markdown += '\n';
  }

  if (!fs.existsSync('keycodes')) {
    fs.mkdirSync('keycodes');
  }
  fs.writeFileSync(path.join('keycodes', name + '.md'), markdown);

  if (name !== 'us') {
    keycode_files.push({ text: convert_name(name), link: '/keycodes/' + name });
  }
}


let template = `export default [
  ${keycode_files.map(item => `{ text: '${item.text == "US" ? "US (Default)" : item.text}', link: '${item.link}' },`).join('\n  ')}
];`;

fs.writeFileSync('_keycodes.mts', template);
