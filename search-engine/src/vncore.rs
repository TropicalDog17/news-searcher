use anyhow::Result;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder, MavenArtifact};
pub struct Pipeline(Instance);
pub struct VnCoreNLP {
    pub jvm: Jvm,
    pub pipeline: Pipeline,
}
impl Pipeline {
    pub fn new(jvm: &Jvm) -> Result<Self> {
        let s1 = InvocationArg::try_from("wseg")?;
        // let s2 = InvocationArg::try_from("pos")?;
        // let s3 = InvocationArg::try_from("ner")?;
        // let s4 = InvocationArg::try_from("parse")?;
        let arr_instance = jvm.create_java_array("java.lang.String", &vec![s1])?;
        let i = InvocationArg::try_from(arr_instance)?;
        let pipeline = jvm.create_instance("vn.pipeline.VnCoreNLP", &[i])?;
        Ok(Pipeline(pipeline))
    }
    pub fn segment(&self, jvm: &Jvm, input: String) -> Result<Vec<String>> {
        let s = InvocationArg::try_from(input)?;
        let annotation: Instance = jvm.create_instance("vn.pipeline.Annotation", &[s])?;
        let annotation2 = jvm.clone_instance(&annotation)?;
        let i_annotation = InvocationArg::try_from(annotation)?;
        let _ = jvm.invoke(&self.0, "annotate", &[i_annotation]);
        let string_instance = jvm.invoke(&annotation2, "toString", &Vec::new())?;
        let rust_string: String = jvm.to_rust(string_instance)?;
        let list = rust_string.replace("\t\t", "\t").replace('_', " ");
        if list.is_empty() {
            return Ok(Vec::new());
        }
        let list = list
            .split('\t')
            .filter(|word| !word.starts_with(' ') && !word.as_bytes()[0].is_ascii_digit())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Ok(list)
    }
}

impl VnCoreNLP {
    pub fn new() -> Result<VnCoreNLP> {
        let entry: ClasspathEntry<'_> = ClasspathEntry::new("VnCoreNLP/VnCoreNLP-1.2.jar");
        let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build()?;
        let pipeline = Pipeline::new(&jvm)?;
        Ok(VnCoreNLP { jvm, pipeline })
    }
    pub fn new_new() -> Result<VnCoreNLP> {
        let entry = ClasspathEntry::new("VnCoreNLP/target/classes");
        let entry2 = ClasspathEntry::new("VnCoreNLP/log4j-1.2.17.jar");
        let jvm: Jvm = JvmBuilder::new()
            .classpath_entry(entry)
            .classpath_entry(entry2)
            .build()?;
        let pipeline = Pipeline::new(&jvm)?;
        Ok(VnCoreNLP { jvm, pipeline })
    }
}
pub fn get_vncorenlp() -> Result<Jvm> {
    let entry: ClasspathEntry<'_> = ClasspathEntry::new("VnCoreNLP/VnCoreNLP-1.2.jar");
    let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build()?;
    Ok(jvm)
}
pub fn get_pipeline_instance(jvm: &Jvm) -> Result<Instance> {
    let s1 = InvocationArg::try_from("wseg")?;
    let arr_instance = jvm.create_java_array("java.lang.String", &vec![s1])?;

    let i = InvocationArg::try_from(arr_instance)?;
    let pipeline = jvm.create_instance("vn.pipeline.VnCoreNLP", &[i])?;
    Ok(pipeline)
}
pub fn test() -> Result<()> {
    let vncore_nlp = VnCoreNLP::new_new()?;
    // Take sentences user input and segment them indefinitely
    loop {
        println!("Please enter a vietnamese sentence");
        let input = read_string();
        if input == *"huhu" {
            break;
        } else {
            vncore_nlp.pipeline.segment(&vncore_nlp.jvm, input)?;
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
mod tests {
    use super::*;

    #[test]
    fn test_simple_segment() {
        let vncore = VnCoreNLP::new_new().unwrap();
        let input = "Tôi là sinh viên trường đại học bách khoa hà nội";
        let result = vncore
            .pipeline
            .segment(&vncore.jvm, input.to_string())
            .unwrap();
        assert_eq!(result.len(), 7);
        assert_eq!(result[0], "Tôi");
        assert_eq!(result[1], "là");
        assert_eq!(result[2], "sinh viên");
        assert_eq!(result[3], "trường");
        assert_eq!(result[4], "đại học");
        assert_eq!(result[5], "bách khoa");
        assert_eq!(result[6], "hà nội");
    }

    #[test]
    fn test_empty_segment() {
        let vncore = VnCoreNLP::new_new().unwrap();
        let input = "";
        let result = vncore
            .pipeline
            .segment(&vncore.jvm, input.to_string())
            .unwrap();
        assert_eq!(result.len(), 0);
    }
    #[test]
    fn test_single_word_segment() {
        let vncore = VnCoreNLP::new_new().unwrap();
        let input = "Tôi";
        let result = vncore
            .pipeline
            .segment(&vncore.jvm, input.to_string())
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Tôi");
    }
    #[test]
    fn test_non_word() {
        let vncore = VnCoreNLP::new_new().unwrap();
        let input = "Tôi là sinh viên trường đại học bách khoa hà nội.";
        let result = vncore
            .pipeline
            .segment(&vncore.jvm, input.to_string())
            .unwrap();
        assert_eq!(result.len(), 8);
        assert_eq!(result[0], "Tôi");
        assert_eq!(result[1], "là");
        assert_eq!(result[2], "sinh viên");
        assert_eq!(result[3], "trường");
        assert_eq!(result[4], "đại học");
        assert_eq!(result[5], "bách khoa");
        assert_eq!(result[6], "hà nội");
        assert_eq!(result[7], ".");
    }
}
