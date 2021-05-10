import sys
from itertools import islice

# convert a string to the inside of a rust string or byte string literal
#
# There are more rules https://doc.rust-lang.org/reference/tokens.html
# but as they're not all needed here, they're not all implemented.
def str_to_rust_literal_chars(input):
    output = ""
    for c in input:
        if c in '\'"\\':
            output += f'\\{c}'
        # if character is in the ascii unprintable range, output \u{hex}
        elif ord(c) < ord(' '):
            output += f'\\u{{{ format(ord(c), "x") }}}'
        else:
            output += c
    return output

class Variable:
    def __init__(self, name, example, comment):
        self.name = name
        self.example = example
        self.comment = comment
    
    def name_as_enum(self):
        in_acronym = False
        capitalize_next = True
        output = ""
        for i in range(len(self.name)):
            c = self.name[i]
            if i+1 >= len(self.name):
                n = None
            else:
                n = self.name[i+1]
            if c == '_':
                capitalize_next = True
            else:
                if capitalize_next:
                    if not in_acronym:
                        output += c.upper()
                        in_acronym = True
                    elif n != '_' and n != None:
                        output += c.upper()
                        in_acronym = False
                    else:
                        output += c
                        in_acronym = True
                else:
                    in_acronym = False
                    output += c
                capitalize_next = False
        return output

    def name_as_c_str_byte_literal(self):
        return f"b\"{str_to_rust_literal_chars(self.name)}\\0\""
    
    def example_as_rust_literal(self):
        return str_to_rust_literal_chars(self.example)
    
    def comment_as_escaped_markdown(self):
        output = ""
        for c in self.comment:
            if c in '\\`*_{}[]()#+-.':
                output += '\\'
            output += c
        return output

variables = []
names = set()

for line in islice(sys.stdin, 1, None):
    (name, example, comment) = line.rstrip('\n').split('\t')
    if not name in names:
        variable = Variable(name, example, comment)
        variables.append(variable)
        names.add(name)

print("// ⚠️ This file is generated")
print("// ⚠️ Regenerate with `make src/variable.rs`")
print()
print("use std::ffi::CStr;")
print()
print("/// Enum representing different variable options accepted by Tesseract")
print("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
print("pub enum Variable {")

for variable in variables:
    print(f"    /// {variable.comment_as_escaped_markdown()}")
    if variable.example != '':
        print(f"    ///")
        print(f"    /// Example value: `\"{variable.example_as_rust_literal()}\"`")
    print(f"    {variable.name_as_enum()},",)

print("}")
print()
print("impl Variable {")
print("    /// Get the variable's name as used by Tesseract")
print("    pub fn as_cstr(&self) -> &'static CStr {")
print("        CStr::from_bytes_with_nul(match self {")

for variable in variables:
    print(f"            Variable::{variable.name_as_enum()} => {variable.name_as_c_str_byte_literal()},")

print("        }).unwrap()")
print("    }")
print("}")
