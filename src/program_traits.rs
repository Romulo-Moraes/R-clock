pub trait StringTrait{
    fn append_char_specified_times(&mut self, the_char : char, times : u16);
}


impl StringTrait for String{
    fn append_char_specified_times(&mut self, the_char : char, times : u16){
        let mut i = 0;

        while i < times{
            self.push(the_char);

            i += 1;
        }
    }
}