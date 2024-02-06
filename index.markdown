---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: home
---

<html>
<head>
<meta http-equiv="Content-Type" content="text/html;charset=utf-8" />
<meta name="viewport" content="width=device-width" />
<title>Regorus Playground</title>
<link
rel="stylesheet"
href="{{'style.css'|relative_url}}"
/>
</head>
<body>
    <h1>Regorus Playground</h1>
    <div class="pane">
        <div class="left-pane">
            <h2>Rego</h2>
            <div id="Rego" class="rego-box"></div>
        </div>

        <div class="right-pane-container">
            <div class="button-container">
                <button id="SelectInput" class="json-button">Input JSON</button>
                <button id="SelectData" class="json-button">Data JSON</button>
                <button id="Eval" class="json-button">Eval</button>
            </div>
            <div class="input-box-container">
                <div id="Input" class="input-box"></div>
                <div id="Data" class="data-box"></div>
            </div>
            <div class="output-box-container">
                <h2>Output</h2>
                <div id="Output" class="output-box"></div>
            </div>
        </div>
    </div>
	
    <script>
      var require = { paths: { vs: "{{'node_modules/monaco-editor/min/vs'|relative_url}}" } };
    </script>
    <script src="{{'node_modules/monaco-editor/min/vs/loader.js'|relative_url}}"></script>
    <script src="{{'node_modules/monaco-editor/min/vs/editor/editor.main.nls.js'|relative_url}}"></script>
    <script src="{{'node_modules/monaco-editor/min/vs/editor/editor.main.js'|relative_url}}"></script>
	
	    <script type="module">



      import init, { Engine } from "{{'pkg/regorus.js'|relative_url}}";

      function loadFile(filePath) {
	      filePath = `{{site.baseurl}}${filePath}`
     	   var result = null;
	       var xmlhttp = new XMLHttpRequest();
	      xmlhttp.open("GET", filePath, false);
	      xmlhttp.send();
	      if (xmlhttp.status==200) {
	        result = xmlhttp.responseText;
	     }
	     return result;
      }


     monaco.languages.register( { id : 'Rego'})

      let rego_language = {
	  //	  defaultToken: 'invalid',

	  keywords: [
	      'as',
	      'contains',
	      'default',
	      'else',
	      'every',
	      'false',
	      'if',
	      'import',
	      'in',
	      'not',
	      'null',
	      'package',
	      'set(',
	      'some',
	      'true',
	      'with',
	  ],
	  

          brackets: [
              ['{','}','delimiter.curly'],
              ['[',']','delimiter.square'],
              ['(',')','delimiter.parenthesis']
          ],

	  operators: [
	      '+', '-', '*', '/', '%',
	      '&', '|',
	      ',', ';', '.',
	      ':',
	      '<', '<=', '=', '==', '>', '>=',
	      '!', '!=',
	  ],

	  // we include these common regular expressions
	  symbols:  /[=><!~?:&|+\-*\/\^%]+/,

	  tokenizer: {
	      root: [
		  [/[a-zA-Z'_\?\\][\w'\?\\]*/, { cases: {'@keywords': 'keyword',
							 '@default' : 'identifier' }}],

		  [':=', 'keyword'],

		  [/"[^\\"]*"/,  'string'],
		  [/`[^\\`]*`/,  'string'],

		  // delimiters and operators
		  [/[{}()\[\]]/, '@brackets'],
		  [/[<>](?!@symbols)/, '@brackets'],


		  [/@symbols/, { cases: { '@operators': 'delimiter',
					  '@default'  : '' } } ],

		  // numbers
		  [/[0-9_]*\.[0-9_]+([eE][\-+]?\d+)?[fFdD]?/, 'number.float'],
		  [/[0-9_]+/, 'number'],

		  // delimiter: after number because of .\d floats
		  [/[;,.]/, 'delimiter'],
		  // whitespace
		  { include: '@whitespace' },

	      ],

	      whitespace: [
		  [/[ \t\r\n]+/, 'white'],
		  [/#.*$/,    'comment'],
	      ],

	  }
      };


      monaco.languages.setMonarchTokensProvider('Rego', rego_language)


      let separator = "\n###POLICY###\n"
      let framework_rego = 'package play'
      var rego_editor = monaco.editor.create(document.getElementById('Rego'), {
	  value: framework_rego,
	  language: 'Rego',
	  minimap: { enabled: false },
      });

      var input_editor = monaco.editor.create(document.getElementById('Input'), {
	  value: '{}',
	  language: 'json',
	  minimap: { enabled: false },
      });

      var data_editor = monaco.editor.create(document.getElementById('Data'), {
	  value: '{}',
	  language: 'json',
	  minimap: { enabled: false },
      });
	  
      var results_editor = monaco.editor.create(document.getElementById('Output'), {
	  value: '{}',
	  language: 'json',
	  readOnly: true,
	  minimap: { enabled: false },
      })

      rego_editor.setValue(loadFile("examples/example.rego"))
      input_editor.setValue(loadFile("examples/input.json"))
	  data_editor.setValue("{}")
      document.getElementById('Data').style.display = 'none'

      function eval_query() {
		  let policy = rego_editor.getValue();
		  let files = policy.split(separator)

          try {
			  var startTime = new Date();
			  var engine = new Engine();
			  for (var i =0; i < files.length; ++i) {
				  engine.add_policy("policy.rego", files[i])
              }

			  engine.set_input(input_editor.getValue());
			  engine.add_data(data_editor.getValue());
			  let parse_time = new Date() - startTime;

              let results = engine.eval_query("data");
    		  var elapsed = new Date() - startTime;
			  let output = `// Evaluation took ${elapsed} milliseconds. parse = ${parse_time}, eval = ${elapsed - parse_time}\n${results}`;
			  results_editor.setValue(output)
          }
		  catch (error) {
		      results_editor.setValue(error)
		  }
	  }
      async function run() {
	  await init()
	  document.getElementById('Eval').onclick = eval_query;
	  document.getElementById('SelectInput').onclick = function() {
	    document.getElementById('Data').style.display = 'none'
	    document.getElementById('Input').style.display = 'inline'
	  }
	  document.getElementById('SelectData').onclick = function() {
	    document.getElementById('Input').style.display = 'none'
	    document.getElementById('Data').style.display = 'inline'
	  }
      }
      run();
    </script>


</body>
</html>
