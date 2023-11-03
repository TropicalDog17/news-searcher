// # Basic Example
//
// This example covers the basic functionalities of
// tantivy.
//
// We will :
// - define our schema
// - create an index in a directory
// - index a few documents into our index
// - search for the best document matching a basic query
// - retrieve the best document's original content.

// ---
// Importing tantivy...
use search_engine::test;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;
fn main() -> tantivy::Result<()> {
    // Let's create a temporary directory for the
    // sake of this example
    let index_path = TempDir::new()?;

    // # Defining the schema
    //
    // The Tantivy index requires a very strict schema.
    // The schema declares which fields are in the index,
    // and for each field, its type and "the way it should
    // be indexed".

    // First we need to define a schema ...
    let mut schema_builder = Schema::builder();

    // Our first field is title.
    // We want full-text search for it, and we also want
    // to be able to retrieve the document after the search.
    //
    // `TEXT | STORED` is some syntactic sugar to describe
    // that.
    //
    // `TEXT` means the field should be tokenized and indexed,
    // along with its term frequency and term positions.
    //
    // `STORED` means that the field will also be saved
    // in a compressed, row-oriented key-value store.
    // This store is useful for reconstructing the
    // documents that were selected during the search phase.
    schema_builder.add_text_field("title", TEXT | STORED);

    // Our second field is body.
    // We want full-text search for it, but we do not
    // need to be able to be able to retrieve it
    // for our application.
    //
    // We can make our index lighter by omitting the `STORED` flag.
    schema_builder.add_text_field("body", TEXT);

    let schema = schema_builder.build();

    // # Indexing documents
    //
    // Let's create a brand new index.
    //
    // This will actually just save a meta.json
    // with our schema in the directory.
    let index = Index::create_in_dir(&index_path, schema.clone())?;

    // To insert a document we will need an index writer.
    // There must be only one writer at a time.
    // This single `IndexWriter` is already
    // multithreaded.
    //
    // Here we give tantivy a budget of `50MB`.
    // Using a bigger memory_arena for the indexer may increase
    // throughput, but 50 MB is already plenty.
    let mut index_writer = index.writer(50_000_000)?;

    // Let's index our documents!
    // We first need a handle on the title and the body field.

    // ### Adding documents
    //
    // We can create a document manually, by setting the fields
    // one by one in a Document object.
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut old_man_doc = Document::default();
    old_man_doc.add_text(title, "Tranh luận về GDragon");
    old_man_doc.add_text(
        body,
        "Không, nó làm căng, nó tự nguyện tới đồn cho công an điều tra, công an khỏi cần triệu tập gì hết, muốn nó xét nghiệm tóc hay nước tiểu gì cũng oke hết. Xong nó cho luật sư lên tiếng khẳng định không có hút chích mai thuý, đồn bậy bạ thì nó kiện. Tụi dân Hàn bất ngờ vì đây là lần đầu thấy nó làm căng như vậy, bên cảnh sát Hàn cũng đang có vẻ là hơi nhùn khi đưa ra thông báo là hiện chưa đủ bằng chứng, còn bên hiệp hội Y Khoa cũng rút đơn tố ông bác sĩ mà theo tin đồn thì là:
        bị nghi ngờ cung cấp ma túy cho nam diễn viên Hàn Quốc Lee Sun Kyun và ca sĩ G-Dragon.
        
        Tất nhiên là theo dõi mấy cái drama bên Hàn thì tốt nhất cứ chờ cho tới chương cuối thôi, ví dụ vụ thằng cựu DBSK ngày xưa lúc đầu cũng lên tiếng khẳng định mình vô tội nhưng sau đó cũng đi tù, hay vụ nhóm 5050 hot hot gần đây tố bị công ty chèn ép nhưng lòi ra là nhóm ba trợn chứ công ty mới bị thiệt hại, hay vụ lùm xùm nhóm AOA có con cựu thành viên tố bị bắt nạt đòi tự sát, cuối cùng fan Hàn quay lưng với con đó.
        
        *Edit: bổ sung thêm cho anh em vozer nào biết tên tuổi thằng này nhưng ít theo dõi, thì dưới tư cách là một người có thể coi là fan của nó, tui thấy vụ này khá thú vị khi mà fan Hàn hiện tại chắc cũng hoang mang không biết tin cái gì bởi vì:
        Tiền sử nó có hút cần sa, vì vậy khán giả Hàn có xu hướng tin thằng này nghiện.
        Xung quanh nó toàn là người nghiện, từ thằng TOP tới các thằng đàn em cùng công ty, càng củng cố niềm tin của khán giả.
        Các hành động của nó khá kì quái, nói năng lắp bắp và phát âm không rõ ràng, bản thân nó cũng là một thằng hút thuốc lá công khai (idol Hàn mà hút thuốc là mệt với tụi fan).
        Vì các lý do trên, tin này vừa nổ ra thì 90% fan Hàn đều đồng tình là thằng này sure kèo mai thuý.
        Bỗng nhiên, lần đầu tiên nó lên tiếng và thuê luật sư làm căng, tuyên bố hùng hồn là bố mày đ hút chích, đ tin bố mày thì chờ bố m xét nghiệm nhé mấy con cho.
        Fan trung lập thấy bất ngờ, động thái mạnh mẽ thế này thì chắc là éo hút, nhưng cách cư xử thì rõ là hút mà???
        Động thái rút đơn kiện + tuyên bố chưa đủ bằng chứng của các cán bộ bên Hàn khiến fan trung lập bỗng dưng thấy kèo bị lật, mình nhớ có 1 top comment là hy vọng kèo này khứa này không hút vì tui vẫn còn muốn nghe nhạc khứa.
        
        Nhìn chung thì vì GD chính là linh hồn BIGBANG (BB) nên cũng y chang như bao drama khác của BB thì fan Hàn đều chung một suy nghĩ: sống như cc nhưng nhạc hay quá nên hy vọng bọn này thoát tội. Thôi thì cứ hạ hồi phân giải.
        ",
    );

    // ... and add it to the `IndexWriter`.
    index_writer.add_document(old_man_doc)?;

    // For convenience, tantivy also comes with a macro to
    // reduce the boilerplate above.
    index_writer.add_document(doc!(
    title => "Những đồ thiết yếu cần lắp cho ô tô",
    body => "Mời các bác vào tham luận cho vui. cái gì nên or không nên lắp, tốt cho sử dụng lâu dài hay theo sở thích cá nhân
    Em trước - Tucson
    Cam hành trình ( em không lắp cam 360 - phụ thuộc công nghệ quá em không thích )
    Cảm biến as lốp
    Bọc vô lăng
    Thảm sàn của Thái
    Nước hoa/ tinh dầu cho trong xe
    Bơm điện tử.
    Dán kính
    
    
    Mời các bácl"
    ))?;

    // Multivalued field just need to be repeated.
    index_writer.add_document(doc!(
    title => "Frankenstein",
    title => "Cảnh sát chưa có bằng chứng xác thực, dư luận Hàn Quốc xoay chiều, ủng hộ G-Dragon",
    body => "ủa tức là thằng này nhận tội rồi nhưng cảnh sát quay xe kêu ko đủ bằng chứng nên chưa khép tội được, hòa cả làng hả
    Trong khi cả cái nhóm này chơi đồ là cái chắc. Gần đây, thông qua luật sư của mình, G-Dragon liên tục phủ nhận việc anh sử dụng ma túy, đồng thời bày tỏ ý định trình diện cảnh sát tự nguyện, có thái độ cứng rắn với những thông tin sai sự thật và mang tính phỉ báng mình.
    Đọc bài báo có vài phút thôi, xin đấy! "
    ))?;

    // This is an example, so we will only index 3 documents
    // here. You can check out tantivy's tutorial to index
    // the English wikipedia. Tantivy's indexing is rather fast.
    // Indexing 5 million articles of the English wikipedia takes
    // around 3 minutes on my computer!

    // ### Committing
    //
    // At this point our documents are not searchable.
    //
    //
    // We need to call `.commit()` explicitly to force the
    // `index_writer` to finish processing the documents in the queue,
    // flush the current index to the disk, and advertise
    // the existence of new documents.
    //
    // This call is blocking.
    index_writer.commit()?;

    // If `.commit()` returns correctly, then all of the
    // documents that have been added are guaranteed to be
    // persistently indexed.
    //
    // In the scenario of a crash or a power failure,
    // tantivy behaves as if it has rolled back to its last
    // commit.

    // # Searching
    //
    // ### Searcher
    //water
    // A reader is required first in order to search an index.
    // It acts as a `Searcher` pool that reloads itself,
    // depending on a `ReloadPolicy`.
    //
    // For a search server you will typically create one reader for the entire lifetime of your
    // program, and acquire a new searcher for every single request.
    //
    // In the code below, we rely on the 'ON_COMMIT' policy: the reader
    // will reload the index automatically after each commit.
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    // We now need to acquire a searcher.
    //
    // A searcher points to a snapshotted, immutable version of the index.
    //
    // Some search experience might require more than
    // one query. Using the same searcher ensures that all of these queries will run on the
    // same version of the index.
    //
    // Acquiring a `searcher` is very cheap.
    //
    // You should acquire a searcher every time you start processing a request and
    // and release it right after your query is finished.
    let searcher = reader.searcher();

    // ### Query

    // The query parser can interpret human queries.
    // Here, if the user does not specify which
    // field they want to search, tantivy will search
    // in both title and body.
    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    // `QueryParser` may fail if the query is not in the right
    // format. For user facing applications, this can be a problem.
    // A ticket has been opened regarding this problem.
    let query = query_parser.parse_query("hàn cảnh sát")?;

    // A query defines a set of documents, as
    // well as the way they should be scored.
    //
    // A query created by the query parser is scored according
    // to a metric called Tf-Idf, and will consider
    // any document matching at least one of our terms.

    // ### Collectors
    //
    // We are not interested in all of the documents but
    // only in the top 10. Keeping track of our top 10 best documents
    // is the role of the `TopDocs` collector.

    // We can now perform our query.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    // The actual documents still need to be
    // retrieved from Tantivy's store.
    //
    // Since the body field was not configured as stored,
    // the document returned will only contain
    // a title.
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    // We can also get an explanation to understand
    // how a found document got its score.
    // let query = query_parser.parse_query("body:river")?;

    // let (_score, doc_address) = searcher
    //     .search(&query, &TopDocs::with_limit(1))?
    //     .into_iter()
    //     .next()
    //     .unwrap();

    // let explanation = query.explain(&searcher, doc_address)?;

    // // println!("{}", explanation.to_pretty_json());
    test();
    Ok(())
}
