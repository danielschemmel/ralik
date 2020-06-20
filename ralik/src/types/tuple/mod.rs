pub fn make_tuple_name(element_types: impl Iterator<Item = impl AsRef<str>>) -> String {
	let mut name = "(".to_owned();
	for (i, element_type_name) in element_types.enumerate() {
		if i > 0 {
			name.push_str(", ");
		}
		name.push_str(element_type_name.as_ref());
	}
	name.push_str(")");

	name.into()
}
