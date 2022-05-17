use std::fs;

pub const MAIN_CONTENT: &str = "import { Commands } from '@doctorts/core';
import { Ping } from './commands/ping/ping.command';

@Commands([new Ping().command()])
export class Main {}

new Main();

";

pub const FILE_EXAMPLE_CONTENT: &str = "import { Command } from '@doctorts/core';

export class Ping {
  @Command('ping')
  command() {
    return 'pong';
  }
}

";

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().chain(c).collect(),
  }
}

pub fn file_content(path: &str, name: &str) {
  let file = format!("{}{}.command.ts", path, &name);

  let file_code: String = format!(
    r#"import {{ Command }} from '@doctorts/core';
    
export class {} {{
    @Command('{}')
    command() {{
        return '{}';
    }}
}}"#,
    some_kind_of_uppercase_first_letter(name),
    name,
    "Response"
  );

  match fs::write(file, file_code) {
    Ok(_) => println!("Command file generated successfully!"),
    Err(erro) => println!("{}", erro),
  }
}

pub fn file_customize_content(path: &str, name: &str) {
  let file = format!("{}/{}.customize.ts", path, name);

  let file_code: String = format!(
    r#"import {{ BaileysEventMap, WASocket }} from "@adiwajshing/baileys";

export function customize{}(command: string) {{
  return (target: any, propertyKey: string, descriptor: PropertyDescriptor) => {{
    return {{
      value: function (...args: any[]): any {{
        const type: keyof BaileysEventMap<any> = "messages.upsert";
        class Call {{
          sock: WASocket;
          constructor(sock) {{
            this.sock = sock;
          }}
          callback = async (m) => {{
            // Build your customization here
            // const msg = m.messages[0];
            // const messageContent: string =
            //   msg.message?.conversation ||
            //   msg.message?.extendedTextMessage?.text;

          }};
        }}
        return {{ type, Call }};
      }},
    }};
  }};
}}
"#,
    some_kind_of_uppercase_first_letter(name),
  );

  match fs::write(file, file_code) {
    Ok(_) => println!("Created command customization file"),
    Err(erro) => println!("{}", erro),
  }
}

pub fn package_json_content(name: &str, version_doctor: &str) {
  let create_package = format!("{}/{}.json", name, "package");

  let package_code: String = format!(
    r#"{{
    "name": "{}",
    "version": "1.0.0",
    "scripts": {{
      "dev": "ts-node-dev src/main.ts"
    }},
    "dependencies": {{
      "@doctorts/core": "^{}",
      "@adiwajshing/baileys": "^4.1.0",
      "qrcode-terminal": "^0.12.0",
      "reflect-metadata": "^0.1.13"
    }},
    "devDependencies": {{
      "@types/node": "^17.0.31",
      "ts-node-dev": "^1.1.8",
      "typescript": "^4.6.4"
    }}
  }}
  "#,
    name, version_doctor
  );

  match fs::write(create_package, package_code) {
    Ok(_) => println!("Created package.json file"),
    Err(erro) => println!("{}", erro),
  }
}

pub const TSCONFIG_JSON: &str = r#"{
  "compilerOptions": {
    "module": "commonjs",
    "declaration": true,
    "removeComments": true,
    "emitDecoratorMetadata": true,
    "experimentalDecorators": true,
    "allowSyntheticDefaultImports": true,
    "target": "es2017",
    "sourceMap": true,
    "outDir": "./dist",
    "baseUrl": "./",
    "incremental": true,
    "skipLibCheck": true,
    "strictNullChecks": false,
    "noImplicitAny": false,
    "strictBindCallApply": false,
    "forceConsistentCasingInFileNames": false,
    "noFallthroughCasesInSwitch": false
  }
}
"#;
