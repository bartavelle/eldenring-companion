from dataclasses import dataclass
from typing import Any, Optional, Tuple
import xml.etree.ElementTree as ET
import sys
import re

tree = ET.parse(sys.argv[1]).getroot()


class Field:
    name: str
    display_name: Optional[str]
    description: Optional[str]
    type: str
    bits: Optional[int]
    array: Optional[int]
    def_value: Any

    def __init__(self, f: ET.Element) -> None:
        definition = f.attrib["Def"].split(" ")
        match definition[0]:
            case "s8":
                self.type = "i8"
            case "u8":
                self.type = "u8"
            case "dummy8":
                self.type = "u8"
            case "s16":
                self.type = "i16"
            case "s32":
                self.type = "i32"
            case "u16":
                self.type = "u16"
            case "u32":
                self.type = "u32"
            case "f32":
                self.type = "f32"
            case _:
                raise ValueError("unsupported type %s" % (definition[0],))
        self.array = None
        self.bits = None
        name = definition[1]
        if ":" in name:
            [name, rbits] = name.split(":")
            self.bits = int(rbits)
        if "[" in name:
            [name, rarray] = name.split("[")
            self.array = int(rarray[:-1])
        self.name = name
        self.display_name = None
        self.description = None
        for c in f:
            if c.tag == "DisplayName":
                self.display_name = c.text
            if c.tag == "Description":
                self.description = c.text
        self.def_value = None
        if len(definition) > 2:
            assert len(definition) == 4
            assert definition[2] == "="
            self.def_value = definition[3]

    def tofield(self) -> str:
        if self.array is not None:
            tp = "[%s;%d]" % (self.type, self.array)
        elif self.bits is not None:
            if self.bits == 1:
                tp = "bool"
            else:
                tp = "Integer<%s, packed_bits::Bits::<%d>>" % (self.type, self.bits)
        else:
            tp = self.type
        name = re.sub(r"__", "", re.sub(r"(?<!^)(?=[A-Z])", "_", self.name).lower())
        return (
            name
            + ": "
            + tp
            + ", // "
            + (self.display_name if self.display_name else "")
            + " - "
            + (self.description if self.description else "")
        )


@dataclass
class FieldState:
    lastbit: Optional[int] = None

    def reset(self):
        self.lastbit = None

    def with_bits(self, b: int) -> Tuple[int, int]:
        if self.lastbit is None:
            self.lastbit = b
            return (0, b)
        else:
            start = self.lastbit
            self.lastbit += b
            return (start, self.lastbit)


print("use packed_struct::prelude::*;")
print("")
print("#[allow(non_camel_case_types)]")
print("#[derive(PackedStruct, Debug, Clone)]")
print('#[packed_struct(endian="lsb", bit_numbering="msb0")]')

for c in tree:
    if c.tag == "ParamType":
        print("pub struct %s {" % (c.text,))
    elif c.tag == "BigEndian":
        if c.text != "False":
            raise ValueError("only support little endian")
    elif c.tag == "Unicode":
        if c.text != "True":
            raise ValueError("only support unicode")
    elif c.tag == "Fields":
        for f in c:
            assert isinstance(f, ET.Element)
            fld = Field(f)
            print("    pub %s" % (fld.tofield(),))

print("}")
