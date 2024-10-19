pub fn json_get_paths(data: &Value, root_symbol: Option<&str>, delimiter: Option<&str>) -> Vec<String> {
    let mut ret: Vec<Vec<String>> = vec![];
    let delimiter: &str = delimiter.unwrap_or(".");
    let root_symbol: String = root_symbol.unwrap_or("$").to_string();
    // let mut parents: HashMap<&Value, String> = HashMap::new();
    // let mut leafs: Vec<&Value> = vec![];
    // let mut curr_key: String;
    // stack.push_back(data);
    for root_key_string in data.as_object().unwrap().keys() {
        let mut acc: Vec<Vec<String>> = vec![];
        let mut stack: VecDeque<&Value> = VecDeque::from([]);
        // let _ = data.as_object().unwrap().values().map(|v| stack.push_back(v));
        acc.push(vec![root_key_string.clone()]);
        stack.push_back(data.as_object().unwrap().get(root_key_string).unwrap());
        while let Some(curr_data) = stack.pop_front() {
            println!("=[json_get_paths]========================================[lead loop]");
            dbg!(root_key_string, &acc, &stack, curr_data);
            if curr_data.is_object() {
                for key_string in curr_data.as_object().unwrap().keys() {
                    let child_data: &Value = curr_data.as_object().as_ref().unwrap().get(key_string).unwrap();
                    if child_data.is_object() {
                        stack.push_back(child_data);
                    }
                    if acc.len() == 0 {
                        acc.push(vec![key_string.clone()]);
                    } else {
                        let mut last_path = acc.last().unwrap().clone();
                        // let mut new_path = vec![key_string.clone()];
                        last_path.append(&mut vec![key_string.clone()]);
                        acc.push(last_path);
                    }
                }
            }
        }
        println!("=[json_get_paths]=========================================[root loop]");
        dbg!(root_key_string, &acc, &stack);
        ret.append(&mut acc);
    }
    dbg!(&ret);
    let mut actual_ret: Vec<String> = vec![];
    actual_ret.push(root_symbol.clone());
    for path_vec in ret {
        let mut buff = vec![root_symbol.clone()];
        buff.append(&mut path_vec.clone());
        actual_ret.push(
            buff.join(delimiter)
        );
    }
    dbg!(&actual_ret);
    return actual_ret;
}