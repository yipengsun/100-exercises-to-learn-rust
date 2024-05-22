pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        let ptr_size = std::mem::size_of::<u64>();
        let length_size = std::mem::size_of::<u64>();
        let capacity_size = std::mem::size_of::<u64>();
        assert_eq!(size_of::<String>(), ptr_size + length_size + capacity_size);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        let string_size = std::mem::size_of::<String>();
        assert_eq!(size_of::<Ticket>(), string_size*3);
    }
}
