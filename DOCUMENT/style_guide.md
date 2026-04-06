## Coding standard

The formatting described below must be treated as a hard rule system, not stylistic preference.

Every single line of code of the codebase must strictly apply the following rules, without any exception.

### Common code rules

All source code files in this repository must follow these rules:

- Four-space indentation: use exactly four spaces per nesting level; do not use tabs for indentation. This applies to C#, D, Dart, JavaScript, Rust, and any other code here.
- Self-documenting code only (prefer descriptive names over explanatory comments).
- Identifiers follow usual English ordering rules (`FirstPointCloudIndex`, `ConfigurationFilePath`, `PointCloudFolderPath`) and use no shortened words, abbreviations (except `Url`) or single-letter names (except for axis names in 3D vectors).
- No plural-form identifiers.
- Aligned braces (Allman style) and keep the opening and closing braces always alone on their own line.
- Each function parameter definition must be on its own line with the closing `)` of the parameter list starting its own line.
- When an arrow function definition spans multiple lines, the arrow `=>` must end its own line, and the function code must be indented on the next line(s).
- When a `return` statement spans multiple lines, the `return` keyword must end its own line, and the return expression value must be indented on the next line(s).
- When the field value after `:` spans multiple lines, the `:` must end its own line, and the field value must be indented on the next line(s).
- When an opening `(` ends its own line, the matching closing `)` must start its own line and be aligned with the first character of the indented argument block.
- When an opening `[` ends its own line, the matching closing `]` must start its own line and be aligned with the first character of the indented element block.
- In multiline boolean expressions, put `&&` and `||` at the start of the continued line, aligned with the first character of its first left operand.
- In multiline arithmetic expressions, put the operator (`+`, etc) at the start of the continued line, aligned with the first character of its first left operand.
- When an assignment spans multiple lines, the assignment operator (`=`, `+=`, etc) must be indented on the next line and start the first line of the right-hand side value.
```
this.age
    += ( age
         + ( age - 2 )
         + ( age
             * ( age + 10 )
             * ( age + 20 ) ) )
       + this.getAgeOffset(
             age * 2
             - 20
             );

final response
    = await http
          .get(
              Uri.parse( 'https://sample.com/message' ),
              headers:
                  {
                      'Accept': 'application/json'
                  }
              )
          .timeout( const Duration( seconds: 5 ) );
```
- For non-empty `()` and `[]`, add spaces inside delimiters: `xxxxx( 10, xxxxx[ 10 ] )`. Keep empty pairs compact: `xxxxx()`, `xxxxx[]`.
- Arrow functions can only be used for getters, field values and call arguments. They can't be used to define the constructors and methods of a class.
- Use standard section comments:
  - `// -- IMPORTS`
  - `// -- CONSTANTS`
  - `// -- TYPES`
  - `// -- ATTRIBUTES`
  - `// -- CONSTRUCTORS`
  - `// -- OPERATORS`
  - `// -- INQUIRIES` (for methods not changing the type data)
  - `// -- OPERATIONS` (for methods changing the type data)
  - `// -- VARIABLES` (for global variables)
  - `// -- FUNCTIONS` (for global functions)
  - `// -- STATEMENTS` (for global statements)
- Standard section comments must only be present for non-empty sections.
- Insert a standard separating comment `// ~~` between types, methods or functions defined within the same standard section.
- Keep an empty line before and after any of those `// -- ...' and `// ~~` standard section and separating comments, unless the standard comment is at the start of a file or `{` block.
- Try to define the code inside sections following the same order as the above section comments, and within those sections, define constants, types, attributes, methods etc before the other elements of code that are using them (i.e. having the used code defined before the code that uses it, allowing for single-read full code understanding).

The following sample code illustrates all the above rules:

```dart
// -- IMPORTS

import 'dart:core';

// -- CONSTANTS

const int
    minimumPassCount = 0,
    maximumPassCount = 5;

// -- TYPES

class Being
{
    // -- ATTRIBUTES

    String
        name;
    int
        age;

    // -- CONSTRUCTORS

    Being(
        this.name,
        this.age
        );
}

// ~~

class Person
    extends Being
{
    // -- ATTRIBUTES

    double
        weight,
        dogCount;

    // -- CONSTRUCTORS

    Person(
        String name,
        int age,
        this.weight,
        this.dogCount
        ) : super( name, age );

    // -- INQUIRIES

    int getAge(
        )
    {
        return age;
    }

    // ~~

    int getAgeOffset(
        int otherAge
        )
    {
        return otherAge - age;
    }

    // ~~

    String getHelloMessage(
        )
    {
        return 'Hello, my name is ${ name }, I\'m ${ age } years old and I weight ${ weight } kilograms.';
    }

    // -- OPERATIONS

    void setAge(
        int age
        )
    {
        this.age = age;
    }

    // ~~

    void setFakeAge(
        int age
        )
    {
        if ( age > 0
             && age < 50
             && ( age < 20
                  || age > 40 ) )
        {
            this.age
                += ( age
                     + ( age - 2 )
                     + ( age
                         * ( age + 10 )
                         * ( age + 20 ) ) )
                   + this.getAgeOffset(
                         age * 2
                         - 20
                         );
        }
        else if ( age > 20
                  && age < 40
                  && ( age < 25
                       || age > 35 ) )
        {
            this.age = ( age * 0.5 ).round();
        }
        else
        {
            this.age = age + 10;
        }
    }
}

// -- FUNCTIONS

Map<String, int>? getAgeInterval(
    List<Person> sortedPersonList
    )
{
    if ( sortedPersonList.isEmpty )
    {
        return null;
    }
    else
    {
        return
            {
                'firstAge': sortedPersonList[ 0 ].age,
                'lastAge': sortedPersonList[ sortedPersonList.length - 1 ].age
            };
    }
}

// -- STATEMENTS

void main(
    )
{
    var passIndex = 0;

    while ( passIndex < 5 )
    {
        ++passIndex;
    }

    do
    {
        ++passIndex;
    }
    while ( passIndex < 10 );

    var personList
        = [
            Person( 'Mike', 49, 85, 1 ),
            Person( 'Luke', 30, 77, 0 ),
            Person( 'John', 30, 72, 3 )
        ];

    personList.sort(
        ( firstPerson, secondPerson )
        {
            try
            {
                if ( firstPerson.age != secondPerson.age )
                {
                    return firstPerson.age - secondPerson.age;
                }
                else
                {
                    return firstPerson.weight.compareTo( secondPerson.weight );
                }
            }
            catch ( error )
            {
                print( error.toString() );
            }

            return 0;
        }
        );

    var ageInterval = getAgeInterval( personList );

    if ( ageInterval != null )
    {
        print( 'First age: ${ ageInterval[ 'firstAge' ] }' );
        print( 'Last age: ${ ageInterval[ 'lastAge' ] }' );
    }
    else
    {
        print( 'No age interval' );
    }
}
```

```dart
// -- IMPORTS

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:http/http.dart' as http;

// -- TYPES

class SampleState
{
    // -- ATTRIBUTES

    final bool
        isBusy;

    // -- CONSTRUCTORS

    const SampleState(
        {
            this.isBusy = false
        }
        );
}

// ~~

class SampleCubit
    extends Cubit<SampleState>
{
    // -- CONSTRUCTORS

    SampleCubit(
        ) : super( const SampleState() );

    // -- OPERATIONS

    void send()
    {
    }
}

// ~~

class SampleWidget
    extends StatelessWidget
{
    // -- ATTRIBUTES

    final String
        message;

    // -- CONSTRUCTORS

    const SampleWidget(
        {
            super.key,
            required this.message
        }
        );

    // -- OPERATIONS

    Future<String> getMessage(
        ) async
    {
        final response
            = await http
                  .get(
                      Uri.parse( 'https://sample.com/message' ),
                      headers:
                          {
                              'Accept': 'application/json'
                          }
                      )
                  .timeout( const Duration( seconds: 5 ) );

        if ( response.statusCode != 200 )
        {
            throw Exception( 'Request failed: HTTP ${response.statusCode}' );
        }

        final decodedJson
            = jsonDecode( response.body ) as Map<String, dynamic>;

        return
            decodedJson[ 'message' ] as String;
    }

    // ~~

    @override
    Widget build(
        BuildContext context
        )
    {
        return BlocBuilder<SampleCubit, SampleState>(
            builder:
                ( BuildContext context, SampleState state )
                {
                    return Column(
                        mainAxisSize: MainAxisSize.min,
                        children:
                            [
                                TextButton(
                                    onPressed:
                                        () =>
                                            debugPrint(
                                                'Message button pressed: $message',
                                                ),
                                    child:
                                        Text(
                                            message,
                                            style: Theme.of( context ).textTheme.labelLarge
                                            )
                                    ),
                                Row(
                                    mainAxisAlignment: MainAxisAlignment.end,
                                    children:
                                        [
                                            TextButton(
                                                onPressed: state.isBusy ? null : () => Navigator.of( context ).pop( false ),
                                                child: const Text( 'Cancel' )
                                                ),
                                            const SizedBox( width: 12 ),
                                            FilledButton(
                                                onPressed:
                                                    state.isBusy
                                                    ? null
                                                    : () => context.read<SampleCubit>().send(),
                                                child:
                                                    state.isBusy
                                                    ? const SizedBox(
                                                        width: 20,
                                                        height: 20,
                                                        child: CircularProgressIndicator( strokeWidth: 2 )
                                                        )
                                                    : const Text( 'Send' )
                                                )
                                        ]
                                    )
                            ]
                        );
                }
            );
    }
}

// -- FUNCTIONS

void main(
    )
{
    runApp(
        MaterialApp(
            home:
                Scaffold(
                    body:
                        Center(
                            child:
                                BlocProvider(
                                    create: ( _ ) => SampleCubit(),
                                    child: const SampleWidget( message: 'Hello' )
                                    )
                            )
                    )
            )
        );
}
```

### Dart code rules

All Dart source code files in this repository must also follow these rules:

- Use `List` or `Map` suffixes matching the collection type (`ProductList`, `productList`, `ProductByIdMap`, `productByIdMap`).
- Use the `Index` suffix for element indices (`ProductIndex`, `productIndex`).
- Use the `Count` suffix for element counts (`ProductCount`, `productCount`).
- Use `UpperCamelCase` for types, annotations, constructors, and type parameters.
- Use `lowerCamelCase` for methods, functions, parameters, locals, fields, and enum values.
- Use `lowercase_with_underscores` for libraries, packages, directories, source files, and import prefixes.
- Blank line before `return`, `if`, `for`, `while`, `for-in`, `switch`, `try`, `catch`, `finally`, and `else` when the previous non-empty line does not end with `{`.
- No trailing commas in argument lists and collection literals.

### C# code rules

All C# source code files in this repository must also follow these rules:

- Use `List` or `Dictionary` suffixes matching the collection type (`ProductList`, `ProductByIdDictionary`, `productList`, `productByIdDictionary`).
- Use the `Index` suffix for element indices (`ProductIndex`, `productIndex`).
- Use the `Count` suffix for element counts (`ProductCount`, `productCount`).
- Use `PascalCase` for namespaces, types, attributes, methods, functions, properties, events, enum members, and public constants.
- Use `_camelCase` for private fields.
- Use `camelCase` for parameters and local variables.
- Blank line before `return`, `if`, `for`, `while`, `foreach`, `switch`, `try`, `catch`, `finally`, `lock`, `do`, and `else` when the previous non-empty line does not end with `{`.

### D code rules

All D source code files in this repository must also follow these rules:

- Use `Array` or `Map` suffixes matching the collection type (`ProductArray`, `ProductByIdMap`, `productArray`, `productByIdMap`).
- Use the `Index` suffix for element indices (`ProductIndex`, `productIndex`).
- Use the `Count` suffix for element counts (`ProductCount`, `productCount`).
- Use `PascalCase` for types, templates, interfaces, enums, and attributes.
- Use `camelCase` for methods, functions, parameters, local variables, and enum members.
- Use `lowercase` (and `lowercase_with_underscores` when a name has multiple words) for modules, packages, and source files.
- Blank line before `return`, `if`, `for`, `while`, `foreach`, `switch`, `try`, `catch`, `finally`, `synchronized`, `do`, and `else` when the previous non-empty line does not end with `{`.

### JavaScript code rules

All JavaScript source code files in this repository must also follow these rules:

- Use `Array` or `Map` suffixes matching the collection type (`ProductArray`, `ProductByIdMap`, `productArray`, `productByIdMap`).
- Use the `Index` suffix for element indices (`ProductIndex`, `productIndex`).
- Use the `Count` suffix for element counts (`ProductCount`, `productCount`).
- Use `PascalCase` for classes, constructors, and public constants at module scope.
- Use `camelCase` for methods, functions, parameters, local variables, object properties, and enum-like members on constant objects.
- Blank line before `return`, `if`, `for`, `while`, `for-in`, `for-of`, `switch`, `try`, `catch`, `finally`, `do`, and `else` when the previous non-empty line does not end with `{`.

### Rust code rules

All Rust source code files in this repository must also follow these rules:

- Use `vector` or `map` suffixes matching the collection type (`product_vector`, `product_by_id_map`).
- Use the `index` suffix for element indices (`product_index`).
- Use the `count` suffix for element counts (`product_count`).
- Use `UpperCamelCase` for types, traits, and enum variants.
- Use `snake_case` for crates, modules, functions, methods, variables, parameters, fields, and macros
- Use `SCREAMING_SNAKE_CASE` for constants and statics.
- Blank line before `return`, `if`, `for`, `while`, `loop`, `match`, and `else` when the previous non-empty line does not end with `{`.
