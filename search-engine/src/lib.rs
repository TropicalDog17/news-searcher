use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
pub fn get_vncorenlp() -> Result<Jvm> {
    let entry = ClasspathEntry::new("VnCoreNLP/VnCoreNLP-1.2.jar");
    let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build()?;
    Ok(jvm)
}
pub fn get_pipeline_instance(jvm: &Jvm) -> Result<Instance> {
    let s1 = InvocationArg::try_from("wseg")?;
    let s2 = InvocationArg::try_from("pos")?;
    let s3 = InvocationArg::try_from("ner")?;
    let s4 = InvocationArg::try_from("parse")?;
    let arr_instance = jvm.create_java_array("java.lang.String", &vec![s1, s2, s3, s4])?;

    let i = InvocationArg::try_from(arr_instance)?;
    let pipeline = jvm.create_instance("vn.pipeline.VnCoreNLP", &[i])?;
    Ok(pipeline)
}
pub fn test() -> Result<()> {
    let jvm = get_vncorenlp()?;
    let pipeline = get_pipeline_instance(&jvm)?;
    loop {
        println!("Please enter a vietnamese sentence");
        let input = read_string();
        if input == *"huhu" {
            break;
        } else {
            let s = InvocationArg::try_from(input)?;
            let annotation: Instance = jvm.create_instance("vn.pipeline.Annotation", &[s])?;
            let annotation2 = jvm.clone_instance(&annotation)?;
            let i_annotation = InvocationArg::try_from(annotation)?;
            let _ = jvm.invoke(&pipeline, "annotate", &[i_annotation]);
            let string_instance = jvm.invoke(&annotation2, "toString", &Vec::new())?;
            let rust_string: String = jvm.to_rust(string_instance)?;
            println!("abc {}", rust_string);
        }
    }
    Ok(())
}
fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}
