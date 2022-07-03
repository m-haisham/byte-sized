macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = u8;

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(v),
                }
            }
        }
    }
}

back_to_enum!(
    #[repr(u8)]
    #[derive(Debug)]
    pub enum OpCode {
        /// Creates and adds the tape where the
        /// state is stored into the stack.
        DefineTape,

        /// Adds the current pointer value to stack.
        PointerValue,

        /// Adds the constant in the defined position to the stack.
        Constant,

        /// Moves the tape pointer to the left.
        MovePointerLeft,

        /// Moves the tape pointer to the right.
        MovePointerRight,

        /// Add one to current pointer value.
        IncrementSingular,

        /// Remove one from current pointer value/
        DecrementSingular,

        /// Write the string starting from current position
        /// to the tape.
        WriteString,

        /// Output the current pointer value.
        Print,

        /// Output the range from the tape.
        PrintRange,

        /// Take input from input provider and
        /// set it to the current pointer cell.
        Input,

        /// Jump to the defined place if stack value is zero.
        JumpIfFalse,

        /// Jump to the defined place (usually before this instruction).
        Loop,

        /// Discard the last added item from stack.
        Pop,

        /// Return the stack value.
        Return,
    }
);

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}
