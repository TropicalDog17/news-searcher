// # Basic Example

// This example covers the basic functionalities of
// tantivy.

// We will :
// - define our schema
// - create an index in a directory
// - index a few documents into our index
// - search for the best document matching a basic query
// - retrieve the best document's original content.

// ---
// Importing tantivy...
use serde::Serialize;
use tantivy::collector::TopDocs;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::*;
use tantivy::tokenizer::*;
use tantivy::{doc, Index, ReloadPolicy};
use tempfile::TempDir;
pub fn query_wrapper(query: String) -> tantivy::Result<Vec<String>> {
    let index_path = TempDir::new()?;

    let text_field_indexing = TextFieldIndexing::default().set_tokenizer("vn_core");
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING | STORED);
    schema_builder.add_text_field("title", text_options.clone());
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("summary", TEXT);
    schema_builder.add_text_field("url", TEXT);
    schema_builder.add_text_field("timestamp", TEXT);

    let schema = schema_builder.build();
    let index: Index = Index::create_in_dir(&index_path, schema.clone())?;
    let vn_tokenizer = VnCore::default();
    index.tokenizers().register("vn_core", vn_tokenizer);
    let mut index_writer = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let content = schema.get_field("content").unwrap();
    let id = schema.get_field("id").unwrap();

    // For convenience, tantivy also comes with a macro to
    // reduce the boilerplate above.
    index_writer.add_document(doc!(
        id => "1",
    title => "Tội ác che giấu 30 năm của kẻ ngoại tình",
    content => r#"Sau vụ mất tích bí ẩn của một công nhân, vợ anh ta cùng nhân tình bỏ trốn, 30 năm sau cảnh sát mới lần ra dấu vết.

    Ngày 2/4/1990, công nhân mỏ than họ Cơ, 32 tuổi, sống ở tỉnh Sơn Tây, bỗng biến mất sau khi tan làm. Khoảng cách giữa mỏ và nhà Cơ không xa nhưng là đường núi khó đi, anh ta thường đi bộ mất nửa tiếng. Nhưng hôm đó gia đình đợi đến khuya cũng không thấy đâu. Bố mẹ Cơ cho biết con trai thường về nhà đúng giờ và không bao giờ qua đêm bên ngoài.
    
    Sáng sớm hôm sau, bố Cơ báo tin con mất tích cho người thân. Ông không cho rằng Cơ đi lạc mà sợ tính cách con quá cứng rắn dễ đắc tội với người khác, bị hãm hại. Cả gia đình chia ra tìm kiếm khắp nơi.
    
    Đồng nghiệp cho biết hôm qua Cơ tan làm đúng giờ như mọi ngày, trước khi rời đi còn nói sẽ về thẳng nhà vì trời mưa sợ bị cảm lạnh. Gia đình đi dọc đường về nhà của Cơ để hỏi thăm nhưng không thu được manh mối nào vì rất ít người ra ngoài khi đó.
    
    Hai ngày trôi qua, Cơ vẫn không có tin tức. Gia đình đoán có chuyện bất trắc xảy ra, nhờ dân làng và thuê người giúp tìm kiếm. Đội ngũ gần 200 người tra xét toàn bộ thôn, đồng ruộng, hầm mỏ, ao hồ, giếng... không bỏ sót chỗ nào. Khi hết cách, gia đình mới trình báo cảnh sát.
    
    Cảnh sát thành phố Cao Bình điều tra tỉ mỉ lộ trình về nhà của Cơ, phát hiện một số vết máu lớn còn mới trên sườn núi phía tây của thôn, lượng máu chảy ra đủ để gây tử vong. Do hôm xảy ra sự việc trời mưa và máu sau khi hòa vào đất sẽ khó nhận ra nên người dân không phát hiện trong quá trình tìm kiếm. Tuy nhiên do công nghệ điều tra tội phạm khi đó chưa phát triển, cảnh sát không thể xác định được máu của ai.
    
    Điều tra dọc theo đoạn đường đó, cảnh sát tìm được một người chăn cừu nói nhìn thấy Cơ vào chiều tối 2/4. Cơ từ trên núi xuống, xin điếu thuốc rồi vội vàng đi. Như vậy, Cơ không thay đổi lộ trình từ mỏ về nhà vào hôm đó. Tuy nhiên, cảnh sát không tìm thấy thêm manh mối giá trị nào.
    
    Khi vụ án bế tắc, một dân làng từng tham gia tìm kiếm Cơ trình báo nhặt được một chiếc đồng hồ dính đất bẩn gần vũng máu. Theo cảnh sát, đây là đồng hồ điện tử kiểu dáng của nữ, không bị hư hại gì ngoài việc trục trặc do dính mưa, rõ ràng vẫn được sử dụng cách đây không lâu.
    
    Đồng hồ điện tử là đồ hiếm ở vùng núi này, sẽ không ai tùy tiện vứt đi. Cảnh sát nhận định đây là manh mối quan trọng để phá án.
    
    Khi cảnh sát yêu cầu gia đình nhận dạng chiếc đồng hồ, em gái Cơ lập tức nhận ra đồng hồ thuộc về chị dâu họ Tần, 35 tuổi, trước đây Tần thường xuyên đeo khi ra ngoài nên cô có ấn tượng sâu sắc.
    
    Cảnh sát định điều tra Tần nhưng đến nhà thì phát hiện cô ta biến mất. Họ nghi Tần sát hại chồng rồi bỏ trốn vì sợ tội. Gia đình Cơ cũng tiết lộ Tần có hành vi khác thường, không hề tỏ ra lo lắng, cũng không tham gia tìm kiếm chồng.
    
    Vợ chồng Cơ đến với nhau sau khi đều trải qua một cuộc hôn nhân. Vợ đầu của Cơ mất sớm, còn Tần ly dị người chồng đầu tiên vì bị bạo hành. Sau nhiều năm độc thân, cả hai kết hôn qua người quen giới thiệu. Tuy nhiên, cuộc sống gia đình họ không hạnh phúc.
    
    Nhà Cơ đông người, sau khi Tần về ở chung xảy ra nhiều xích mích, cả hai thường xuyên cãi nhau vì những chuyện nhỏ nhặt, sống ly thân một năm qua.
    
    Qua thẩm vấn, cảnh sát được biết từ khi Tần và Cơ còn chung sống, cô ta đã ngoại tình với một người đàn ông cùng làng họ Lục, 37 tuổi. Cả hai thường hẹn hò trên núi, dù bị dân làng bắt gặp cũng không chấm dứt quan hệ. Việc này lan truyền khắp thôn, Tần và Lục bị mắng chửi, Cơ cũng bị chế giễu.
    
    Hai vợ chồng từng cãi vã vài lần nhưng Tần đã rơi vào lưới tình nên làm ngơ những lời trách móc của chồng, chỉ muốn chung sống với Lục. Cơ giận dữ ly thân từ đó, Tần bỏ về nhà mẹ đẻ.
    
    Cảnh sát tìm kiếm Lục nhưng anh ta cũng biến mất. Hàng xóm cho biết Lục dẫn Tần đi vào ngày cảnh sát đến điều tra. Mọi người nghĩ họ đi hẹn hò như nhiều lần trước nên không quan tâm. Một ngày trước khi Lục biến mất, hàng xóm thấy mặt anh ta có những vết xước như bị ai cào, nhưng anh ta nói bị cành cây va quệt khi lên núi làm việc.
    
    Cảnh sát kết luận Lục và Tần có liên quan mật thiết đến vụ án. Sau khi bỏ lỡ cơ hội bắt giữ nghi phạm, họ nhanh chóng liên lạc với cảnh sát các khu vực xung quanh để truy tìm tung tích hai người. Tuy nhiên, cuộc điều tra gặp nhiều khó khăn do công nghệ hạn chế, chưa có hệ thống dữ liệu dân cư.
    
    Suốt 30 năm, vụ án vẫn chưa được giải quyết. Cơ sống chết không rõ, Tần và Lục biến mất. Đến năm 2020, vụ án mới có bước đột phá.
    
    Cảnh sát nhận định ngoại hình của hai người đã thay đổi rất nhiều trong thời gian qua và có thể đã thay đổi danh tính, không thể tìm thấy bằng các phương pháp thông thường. Từ thông tin Tần bỏ lại con gái 9 tuổi họ Viên với người chồng đầu tiên khi chạy trốn, cảnh sát tìm đến Viên, hiện đã 40 tuổi, để tìm hiểu Tần có bí mật liên lạc với gia đình không.
    
    Trước cảnh sát, Viên lộ vẻ lưỡng lự như muốn nói gì đó. Cảnh sát khuyên nhủ: "Chắc chắn Tần sống ở bên ngoài không hề dễ dàng, đi đâu làm gì cũng sợ bị bắt. Chuyện trôi qua 30 năm rồi, đã đến lúc phải kết thúc. Chỉ khi đầu thú bà ấy mới được khoan hồng".
    
    Viên khóc, khai báo hành tung của mẹ cho cảnh sát, đồng thời thuyết phục mẹ ra đầu thú vào ngày 16/7/2020. Ngay sau đó, cảnh sát bắt Lục. Cả hai thú nhận tội ác và thuật lại quá trình lẩn trốn.
    
    Sáng 2/4/1990, Tần lên sườn núi phía tây thôn để khai hoang. Buổi trưa, cô ta đến nhà Lục, rủ chiều lên núi hẹn hò. Không may trời mưa to, cả hai phải chui xuống gầm cầu tránh mưa. Khi đang tán tỉnh, đôi tình nhân bỗng bị ném đá. Tần ra xem thì thấy Cơ đứng gần đó, giận dữ trừng mắt nhìn mình.
    
    Hóa ra Cơ đi làm về qua cầu, chợt nghe thấy tiếng nói chuyện, sau đó phát hiện vợ cùng tình nhân đang trốn dưới gầm cầu. Anh ta tức giận nhặt đá ném. Bị bắt quả tang ngoại tình, Lục không thấy chột dạ mà khó chịu vì bị ném trúng người nên xông tới đánh nhau với Cơ.
    
    Tần lo lắng khuyên can nhưng hai người càng đánh càng ác liệt. Một lúc sau cả hai mới dừng tay, chuyển sang mắng chửi. Tần bực bội về nhà trước.
    
    Sau khi Tần đi, hai người đàn ông lại lao vào đánh nhau. Bị Cơ đấm vào mặt, Lục tức giận nhặt đá đập mạnh vào đầu tình địch. Chưa hết giận, hắn tiếp tục đập bốn năm phát nữa khiến Cơ tử vong tại chỗ.
    
    Cây cầu đá trên núi là nơi xảy ra án mạng. Ảnh: Toutiao
    Cây cầu đá trên núi là nơi xảy ra án mạng. Ảnh: Toutiao
    
    Tần lo Lục bị thương nên quay lại xem tình hình, phát hiện chồng bị đánh chết thì sợ hãi cùng Lục chôn giấu thi thể ở ngọn núi gần thôn. Khi gia đình Cơ báo cảnh sát, biết sẽ bị tra ra nên Lục quyết định đưa Tần đi trốn.
    
    Trong 30 năm qua, cả hai sống chật vật, chạy đến những nơi xa xôi hẻo lánh để tránh bị phát hiện, làm thuê mưu sinh. Vì không có chứng minh thư, họ không thể đi tàu xe, bị bệnh cũng không thể đến bệnh viện.
    
    Không chịu được nỗi nhớ nhà, Tần bí mật liên lạc với con gái, thậm chí còn lén quay về gặp con. Thời gian trôi qua, Tần vô cùng hối hận về hành động năm xưa. Viên cũng nhận ra mẹ do dự nên khi cảnh sát đến, cô quyết định khuyên mẹ đầu thú.
    
    Lục chỉ nơi chôn giấu thi thể Cơ cho cảnh sát. Ảnh: ifeng
    Lục chỉ nơi chôn giấu thi thể Cơ cho cảnh sát. Ảnh: ifeng
    
    Ngày 22/7/2020, cảnh sát khai quật hài cốt của Cơ trên núi. Nạn nhân được yên nghỉ sau 30 năm, còn Tần và Lục phải nhận trừng phạt của pháp luật."#
    ))?;

    index_writer.add_document(doc!(
        id => "2",
    title => "Đại lộ Đông Tây - 'con đường di sản' của TP HCM",
    content => r#"
    Với chiều dài 24 km qua địa bàn 8 quận huyện, đại lộ Đông Tây (Võ Văn Kiệt - Mai Chí Thọ) được đánh giá là con đường "dài 300 năm" bởi nó chạy suốt chiều dài lịch sử hình thành và phát triển vùng đất Sài Gòn - TP HCM.
    > Đề xuất 'biến' khu Chợ Lớn thành phố cổ
    
    Phó chủ tịch UBND TP HCM Nguyễn Hữu Tín vừa phê duyệt nhiệm vụ thiết kế đô thị và quy định quản lý không gian, kiến trúc, cảnh quan trục Đại lộ Đông Tây, con đường được cho là đẹp và hiện đại nhất TP HCM. Khu vực nghiên cứu kéo dài từ ngã 3 Cát Lái (quận 2) đến điểm giao cắt quốc lộ 1A với cao tốc TP HCM - Trung Lương, qua khu vực các quận 1, 2, 4, 5 , 6 , 8, Bình Tân và huyện Bình Chánh với chiều dài hơn 24 km và diện tích hơn 1.500 ha.
    
    Nội dung chính của quyết định nhằm xây dựng không gian đô thị dọc trục đường theo hướng phát triển đô thị hiện đại, phát triển các cụm công trình nhà ở kết hợp chức năng thương mại, dịch vụ, phát huy tốt lợi thế về giao thông, cải tạo không gian và cảnh quan môi trường đô thị.
    
    
    Đại lộ Đông Tây dài hơn 24 km được đánh giá là "con đường di sản" chạy suốt chiều dài lịch sử hơn 300 năm hình thành và phát triển của vùng đất Sài Gòn TP HCM. Ảnh: Hữu Công.
    
    Theo Sở Quy hoạch Kiến trúc TP HCM, con đường này đi qua 4 khu vực đô thị với những đặc thù riêng biệt. Đầu tiên là đô thị mới Thủ Thiêm nằm ở phía quận 2, kế đến là trung tâm hành chính văn phòng lâu đời nằm ở quận 1. Tiếp theo là trung tâm buôn bán, kinh doanh mang sắc thái người Hoa ở quận 5 và cuối cùng là vùng cảnh quan sông nước mang đậm dấu ấn "trên bến dưới thuyền" một thời nhộn nhịp kinh doanh sầm uất ở quận 6 và quận 8.
    
    Khu đô thị mới Thủ Thiêm sẽ có những kiến trúc hiện đại. Khu quận 1 sẽ lưu giữ lại một số kiến trúc Pháp tiêu biểu cho một Sài Gòn xưa. Một số kiến trúc hiện đại ở đây sẽ được nghiên cứu cho hài hoà với không gian chung. Khu vực quận 5 và cả quận 6, 8 là các hoạt động thương mại nhộn nhịp gắn với kênh Tàu Hũ - Bến Nghé (con kênh chạy suốt trục đường) của cả người Việt lẫn người Hoa. Dự kiến ở đây sẽ hình thành một khu chợ nổi để vừa phục vụ nhu cầu kinh doanh buôn bán hiện tại, vừa tái hiện không gian của những ngày đầu hình thành đô thị Sài Gòn.
    
    Trao đổi với VnExpress.net, TS Nguyễn Anh Tuấn, Phó giám đốc Trung tâm Nghiên cứu kiến trúc (Sở Quy hoạch Kiến trúc TP HCM), tác giả đồ án nghiên cứu thiết kế đô thị Đại lộ Đông Tây cho biết dự án Đại lộ Võ Văn Kiệt - Mai Chí Thọ là dự án điểm, có ý nghĩa quan trọng đối với sự phát triển kinh tế xã hội của TP HCM, kết nối trục giao thông chính, hành lang Đông - Tây của thành phố.
    
    
    Khu nhà cổ bột giặt Net đang bị xuống cấp được đề xuất bảo tồn. Ảnh: Hữu Công.
    
    Đại lộ Võ Văn Kiệt - Mai Chí Thọ và hành lang đô thị dọc theo trục đường này, với chức năng quan trọng về giao thông và những giá trị văn hóa, xã hội, kiến trúc đô thị và cảnh quan thiên nhiên đặc thù cần được bảo tồn và phát huy giá trị như trụ sở ngân hàng, Sở Giao dịch chứng khoán, khu nhà cổ bột giặt Net và các đình, chùa, miếu.... Tuy nhiên, không gian kiến trúc cảnh quan hiện nay chưa tương xứng với vai trò của trục đường.
    
    "Nhu cầu phát triển đô thị tại các khu đất dọc trục đại lộ là rất lớn trong khi nhiều quỹ đất dọc trục đường (như các khu vực nhà xưởng, nhà kho sau khi di dời sản xuất ra ngoại thành) chưa được nghiên cứu một cách toàn diện và đồng bộ nhằm phát huy tối đa hiệu quả quỹ đất", ông Tuấn cho biết.
    
    Cũng theo ông Tuấn, nhu cầu xây dựng mới, chỉnh trang nhà ở của nhân dân tại các khu dân cư dọc trục đường chưa có sự hướng dẫn quy định cụ thể của cơ quan quản lý. Công trình, cụm công trình có giá trị lịch sử, văn hóa, kiến trúc chưa được nghiên cứu cụ thể bảo vệ giá trị. Hệ thống hạ tầng kỹ thuật, môi trường tự nhiên trên trục đường chưa có quy hoạch hoàn chỉnh. Các vấn đề tồn tại này đặt ra sự cần thiết nghiên cứu lập thiết kế đô thị trục Đại lộ Võ Văn Kiệt là cơ sở để UBND các cấp tổ chức lập Quy chế quản lý không gian, kiến trúc, cảnh quan tại từng khu vực riêng của trục đường trong ranh giới hành chính do mình quản lý.
    
    
    Kênh tàu hủ chạy qua địa bàn các quận 5, 6 và 8, dự kiến ở đây sẽ hình thành một khu chợ nổi để vừa phục vụ nhu cầu kinh doanh buôn bán hiện tại, vừa tái hiện không gian của những ngày đầu hình thành đô thị Sài Gòn. Ảnh: Hữu Công.
    
    Theo nhiệm vụ thiết kế đô thị được UBND thành phố phê duyệt, Đại lộ Đông Tây được xác định là trục đường cửa ngõ của thành phố, yêu cầu là trục đường đẹp, văn minh, hiện đại, khai thác tối đa các yếu tố cảnh quan sông nước...
    
    Đồ án cũng đề xuất tăng cường giao thông công cộng dọc tuyến đường, bao gồm các tuyến đường thủy bộ, (có phục vụ du lịch trên sông), tuyến giao thông vận chuyển hành khách công cộng tốc độ nhanh, số lượng người lớn (tuyến tramway, tàu điện ngầm, hoặc xe buýt nhanh) và các tuyến đi bộ. Nghiên cứu đầu tư hệ thống xe buýt tốc độ cao dọc theo trục đường, với cự ly bến dừng, bãi đậu xe hợp lý - có kết hợp với các trung tâm thương mại dịch vụ, công trình công cộng và nhà ở cao tầng.
    
    Hiện Sở Quy hoạch Kiến trúc đang gấp rút chỉ đạo Trung tâm Nghiên cứu Kiến trúc triển khai hoàn tất nội dung đồ án, bao gồm bước làm việc, lấy ý kiến với các Sở Ban ngành và các quận, huyện trên địa bàn liên quan để hoàn chỉnh hồ sơ và soạn thảo quy định quản lý.
    
    
    Ngoài Đại lộ Đông Tây, UBND TP HCM cũng đã chỉ đạo Sở Quy hoạch Kiến trúc nghiên cứu thiết kế đô thị 2 tuyến đường quan trọng khác là xa lộ Hà Nội và Tân Sơn Nhất - Bình Lợi. Theo Sở này, xa lộ Hà Nội là cửa ngõ nhộn nhịp vào bậc nhất của TP HCM vấn đề chính ở đây là tìm kiếm ý tưởng thiết kế đô thị có thể làm giảm đi tác động xấu của cường độ giao thông lớn lên cuộc sống của người dân, tạo điều kiện cho người dân tiếp cận thuận tiện với giao thông nhưng vẫn an toàn, tiện lợi.
    
    Còn đường Tân Sơn Nhất - Bình Lợi, đây là trục giao thông quan trọng nối sân bay Tân Sơn Nhất đi ra cửa ngõ phía Đông Bắc của TP HCM. Tuyến đường đi qua nhiều khu vực dân cư chật chội, cũ kỹ, xuống cấp của thành phố thuộc các quận Gò Vấp, Bình Thạnh... nên nhiệm vụ chính của thiết kế đô thị dọc tuyến đường chính là chỉnh trang đô thị, cải tạo đô thị cũ, xây dựng đô thị hiện đại hơn."#,
    ))?;
    index_writer.add_document(doc!(
        id => "3",
    title => "Vụ án 70 mảnh thi thể dưới chân cầu rúng động nước Anh",
    content => r#"
    Năm 1935, cảnh sát phát hiện 70 mảnh thi thể rải rác trong khe núi, các bộ phận nhận dạng quan trọng đều bị vứt bỏ, vết cắt gọn gàng cho thấy thủ phạm là kẻ chuyên nghiệp.

Ngày 29/9/1935, một du khách đang đi bộ ở thị trấn Moffat, gần biên giới Scotland, nhìn thấy gói đồ kỳ lạ nằm cạnh dòng suối trong khe núi bên dưới cây cầu. Nhô ra từ đó là một cánh tay.

Sau nỗ lực rà soát nhiều ngày, cảnh sát tìm thấy 70 mảnh thi thể, nghi của hai người. Vụ án trở thành thách thức đặc biệt với các cơ quan chức năng bởi thủ phạm khôn ngoan đã loại bỏ các yếu tố nhận dạng quan trọng nhất, trong đó có dấu vân tay.

Các thám tử đã kêu gọi các giáo sư pháp y nổi tiếng của Đại học Edinburgh và Glasgow ghép lại các bộ phận này. Họ nhanh chóng nhận ra kẻ giết người ắt hẳn có kiến thức chuyên môn về giải phẫu.

Các nhà điều tra thu gom những phần thi thể phục vụ công tác pháp y. Ảnh: Daily record
Các nhà điều tra thu gom những phần thi thể phục vụ công tác pháp y. Ảnh: Daily record

Từng có những vụ án giết người liên quan việc phân xác, nhưng chưa bao giờ bộ phận của hai thi thể lại bị trộn lẫn với nhau như thế này. Điều này khiến việc ráp lại và nhận dạng trở nên gần như không thể. Hai giáo sư giải phẫu học, dù vậy vẫn có thể xác định nạn nhân là hai phụ nữ. Một người tầm 35-45 tuổi, có 5 vết đâm vào ngực, gãy nhiều xương và nhiều vết bầm tím. Phổi của cô bị tắc nghẽn đáng kể cho thấy cô bị siết cổ trước khi các vết thương khác xảy ra.

Thi thể thứ hai là phụ nữ 20-21 tuổi vào thời điểm bị sát hại, tay chân và đầu có dấu hiệu bị chấn thương. Thời gian phân xác mất khoảng 8 tiếng và nạn nhân bị vứt dưới cầu 12-14 ngày.

Mốc thời gian này được củng cố thêm khi các nhà điều tra nhận thấy những phần hài cốt được bọc trong ga giường, vỏ gối, quần áo trẻ em trên và những trang giấy ngày 15/9 của tờ báo Sunday Graphic, tức cách 14 ngày. Đây là một ấn bản lưu hành chỉ ở khu vực Morecambe và Lancaster. Manh mối này gợi ý mạnh mẽ rằng hai nạn nhân và/hoặc kẻ giết người sống ở thành phố này.

Theo linh cảm, cảnh sát trưởng phụ trách vụ án nhấc điện thoại gọi đến đồn cảnh sát Lancaster.

Tại Lancaster lúc này, một vụ mất tích đã được báo cáo cách đó 5 ngày. Người báo tin là bác sĩ nổi tiếng địa phương, Buck Ruxton. Ông xuất thân từ gia đình giàu có ở Ấn Độ, mẹ là người Pháp. Đến Anh học y năm 1927, ông sau đó cưới vợ, Isabella và có 3 con, được dân chúng yêu quý vì thường chữa bệnh miễn phí cho người nghèo.

Ông cho hay vợ đã "mất tích" chục ngày nay với người hầu gái, Mary. Ông cáo buộc vợ đã ngoại tình, có thai và lén mang cô hầu đi sang thành phố khác để phá thai, bỏ lại 4 cha con bơ vơ, nay vẫn chưa về.

Bác sĩ Buck Ruxton. Ảnh: BBC
Bác sĩ Buck Ruxton. Ảnh: BBC

Sau khi xác dưới chân cầu được phát hiện, lần thứ hai bác sĩ tìm đến đồn cảnh sát. Ông bật khóc và than thở vì có tin đồn hai thi thể chính là vợ và cô hầu. Dư luận đang chĩa búa rìu vào ông, nghi là thủ phạm khiến ảnh hưởng đến thanh danh và hoạt động của phòng khám.

Bác sĩ yêu cầu cảnh sát tiến hành các cuộc điều tra "kín đáo" để dập tắt tin đồn này. Dù Buck Ruxton đã được các cảnh sát xoa dịu trước khi được đưa về nhà, nhưng tại thời điểm này, tất cả đội ngũ điều tra đều xác định, ông ta là nghi phạm chính.

Tối 12/10, Buck Ruxton bị Cảnh sát Lancaster bắt giữ và thẩm vấn suốt đêm. Ông khẳng định, chưa từng đến Scotland, nơi tìm thấy 70 mảnh thi thể, kể từ khi đến nước Anh.

Tuy nhiên, ông ta không thể giải thích lý do tại xe của mình lại bị ghi vé phạt ở Cumbrian, Scotland ngày 17/9. Theo nội dung vi phạm, ôtô của bác sĩ tông vào một người đi xe đạp, sau đó bỏ chạy. Nhưng nạn nhân nhanh chóng ghi lại biển số xe và báo cảnh sát, nên ngay sau đó ông bị bốt cảnh sát ở Milnthorpe, cách đó 10 km, chặn lại ghi vé phạt.

Trong khi đó, báo chí Anh đã phát cuồng về vụ bắt giữ vị bác sĩ Ấn Độ đẹp trai, nổi tiếng. Các nhà điều tra vẫn âm thầm thu thập chứng cứ.

Các giáo sư pháp y đã dành nhiều ngày để kiểm tra vết máu tại nhà Buck Ruxton trên quảng trường Dalton ở Lancaster. Việc di chuyển liên tục giữa Scotland và Lancaster buộc họ phải chuyển các thành phần cấu trúc chính của ngôi nhà đến các phòng thí nghiệm ở Edinburgh, bao gồm cả bồn tắm dành cho bác sĩ và toàn bộ cầu thang, sau đó lắp đặt lại trong phòng thí nghiệm.

Họ phát hiện ra nhiều vết máu trên cầu thang, lan can và nhiều loại thảm khác nhau trong nhà, dù rõ ràng ngôi nhà đã được dọn dẹp kỹ càng và một số bức tường xung quanh, cầu thang vừa được trang trí lại. Dấu vết của mỡ và mô cơ thể người cũng được tìm thấy trong đường cống của khu nhà, phần lớn trong đoạn cống dẫn thẳng từ phòng tắm.

Giáo sư pháp y John Glaister Jnr (trái) của đại học Glasgow và các cộng sự, người đóng vai trò to lớn trong giải quyết vụ án. Ảnh: Sunay Mail
Giáo sư pháp y John Glaister Jnr (trái) của đại học Glasgow và các cộng sự, người đóng vai trò to lớn trong giải quyết vụ án. Ảnh: Sunay Mail

Trong khi đó, cảnh sát Scotland và Lancaster hợp tác chặt chẽ, thu thập càng nhiều dấu vân tay càng tốt để cố gắng đi tìm sự trùng lặp với những dấu vân tay được lấy từ hài cốt được tìm thấy ở khe núi.

Rạng sáng 13/10, dấu vân tay và lòng bàn tay trên bộ bàn tay các nạn nhân được kiểm chứng trùng khớp với dấu ấn trên những món đồ mà vợ và hầu gái mất tích thường sử dụng tại nhà.

Ngoài ra, với thi thể được xác định là vợ bác sĩ, các nhà pháp y đã sử dụng kỹ thuật nhân chủng học pháp y. Ảnh chụp X-quang hộp sọ của nạn nhân được chồng lên bức ảnh chụp Isabella khi còn sống và cho thấy sự trùng khớp cao.

Họ cũng tạo ra mô hình bản sao bàn chân trái của hai nạn nhân bằng hỗn hợp gelatin-glycerin dẻo. Khi được đặt vào đôi giày mà những người phụ nữ đã đi khi còn sống, bàn chân bản sao của 2 nạn nhân vừa khít với 2 chiếc giày mà Isabella và cô hầu Mary đi lúc còn sống, lần lượt tương đương cỡ 36 và 39.

Cảnh sát cực kỳ hài lòng vì họ có đủ bằng chứng để đối thoại với bác sĩ. Còn vị bác sĩ không tìm ra lời nào để giải thích cho những bằng chứng này, ngoài câu: "Tôi không biết, không nhớ".

Phiên tòa bom tấn

Buck Ruxton bị khởi tố hai tội danh Giết người. Phiên xử được ấn định, vào ngày 2/3/1936, Tòa án Công lý Tối cao Manchester, thực sự đã trở thành một "sự kiện bom tấn" bao trùm tin tức quốc gia.

Bất chấp thời tiết giá rét, suốt 12 ngày diễn ra phiên tòa, hơn 5.000 người dân đã tập trung dưới đường, xếp hàng bên ngoài tòa án, chen lấn để giành một vị trí vào trong phòng xử. Có người đứng xếp hàng từ 10h tối hôm trước. Trong khi dân vùng khác mang theo đồ ăn và chăn để ngủ luôn tại đường phố quanh Tòa suốt 12 ngày. Các phóng viên, tay cầm thẻ báo chí và sổ ghi chép, xếp hàng vào phòng xử án.

Buck đã thuê luật sư bào chữa hình sự giỏi nhất nước, nhưng chính luật sư này cũng khá lo lắng vì các chứng cứ buộc tội rất thuyết phục. Song luật sư cũng bám víu vào lập luận, rằng những phương thức phá án như dựa vào hình chụp X-quang, dấu chân, thời gian hình thành của giòi... còn quá mới và chưa được công nhận tính xác thực, do đó, không đáng tin cậy.

Trong khi đó, cơ quan công tố cáo buộc, tối 14/9, Isabella trở về nhà lúc 23h, sau khi cùng hai chị gái đi dự một sự kiện lễ hội địa phương. Vị bác sĩ có hình ảnh xã hội rất tốt, nhưng nổi tiếng ghen tuông vô lối, kiểm soát, đã buộc tội vợ đi với nhân tình. Theo những người hầu làm chứng, đêm đó vợ chồng ông chủ cãi nhau lớn.

Dựa theo kết quả pháp y về việc nạn nhân bị siết cổ trước khi hình thành các tổn thương khác, nhà chức trách cho rằng bác sĩ Buck đã bóp cổ vợ, sau đó đánh đến chết. Cô hầu gái Mary, vô tình chứng kiến sự việc, trở thành nạn nhân thứ hai.

Hôm sau, bác sĩ gửi ba đứa con của mình cho một người bạn, còn mình dành cả ngày trong phòng tắm khóa trái cửa. Cơ quan công tố cáo buộc, khoảng thời gian này, Buck phân xác vợ trong bồn tắm bằng dao phẫu thuật, kéo dài khoảng 8 giờ.

Ông ta sau đó chở các mảnh thi thể bằng ôtô, đến Scotland, cách đó 2 giờ lái xe. "Thảm, sàn nhà và ghế xe đều có vết máu, dù anh ta cố gắng cọ rửa nó rất kỹ càng", công tố viên cáo buộc.

Hơn 100 chuyên gia và người thân các nạn nhân và có liên quan đã ra tòa làm chứng. Những bằng chứng cảnh sát và các nhà khoa học pháp y, được thu thập bằng những kỹ thuật chưa từng được sử dụng, đã làm nên lịch sử, khi được tòa chấp thuận. Sau chưa đến nửa giờ, bồi thẩm đoán tuyên Buck Ruxton có tội, thẩm phán tuyên phạt tử hình.

Cảnh sát ngăn chặn đám đông trước khu vực hành quyết bác sĩ Buck Ruxton, ngày 12/5/1936. Ảnh: Manchester evening news
Cảnh sát ngăn chặn đám đông trước khu vực hành quyết bác sĩ Buck Ruxton, ngày 12/5/1936. Ảnh: Manchester evening news

Hơn 10.000 chữ ký của "fan hâm mộ" bác sĩ này được gửi tới tòa xin giảm án, song không được chấp thuận. Sáng 12/5/1936, ông ta bị treo cổ.

Một ngày sau, tờ News of the World đã đăng một bản thú tội viết tay ngắn gọn, do Buck Ruxton viết một ngày sau khi bị bắt. Lời thú tội ông ta đưa cho người bạn thân, dặn chỉ được mở trong trường hợp mình bị hành quyết hoặc trả lại cho ông ta nếu được trắng án. Trong thư viết tay, bác sĩ Buck thừa nhận đã giết vợ trong lúc ghen tuông, nhưng bị cô hầu can thiệp vào nên buộc phải giết cả hai để bịt đầu mối, đúng như cáo buộc của công tố viên.

Sau cái chết của cha mẹ, 3 đứa con của họ được giấu danh tính và chuyển vào một trại trẻ mồ côi, hồ sơ về chúng chỉ được giải mật và công bố 100 năm sau bản án, tức năm 2035.

Bất chấp sự tìm kiếm ráo riết của cảnh sát, phần thân của hầu gái Mary Jane không bao giờ được tìm thấy.

Ngôi nhà trên Quảng trường Dalton nơi xảy ra vụ án mạng khét tiếng bị bỏ hoang nửa thế kỷ. Năm 1980, nó được tái tạo gần như toàn bộ, trở thành văn phòng kiến trúc sư."#))?;

    index_writer.commit()?;
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
    // in both title and content.
    let query_parser = QueryParser::for_index(&index, vec![title, content]);

    // `QueryParser` may fail if the query is not in the right
    // format. For user facing applications, this can be a problem.
    // A ticket has been opened regarding this problem.
    let query = query_parser.parse_query(&query)?;

    // A query defines a set of documents, as
    // well as the way they should be scored.
    //

    // We can now perform our query.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    let mut result: Vec<String> = Vec::new();

    let a = top_docs.len();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        result.push(schema.to_json(&retrieved_doc));
    }
    Ok(result)
}
use crate::article::Article;
use crate::VnCoreNLP;
use unicode_segmentation::UnicodeSegmentation; // 1.6.0

#[derive(Clone, Default)]

// Tokenizer for search engine, calling VNCoreNLP Java lib
struct VnCore {
    token: Token,
}
pub struct SimpleTokenStream<'a> {
    text: &'a str,
    token: &'a mut Token,
    segmented_text: Vec<String>,
}

impl Tokenizer for VnCore {
    type TokenStream<'a> = SimpleTokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> SimpleTokenStream<'a> {
        self.token.reset();
        let vncore_nlp = VnCoreNLP::new_new().unwrap();
        let segmented_text = vncore_nlp
            .pipeline
            .segment(&vncore_nlp.jvm, text.to_string())
            .unwrap();
        SimpleTokenStream {
            text,
            token: &mut self.token,
            segmented_text,
        }
    }
}
impl<'a> TokenStream for SimpleTokenStream<'a> {
    fn advance(&mut self) -> bool {
        self.token.text.clear();
        self.token.position = self.token.position.wrapping_add(1);

        // advance based on segmented text
        if self.segmented_text.is_empty() {
            self.token.text = self.segmented_text.remove(0);
            return true;
        }
        false
    }

    fn token(&self) -> &Token {
        self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use tantivy::tokenizer::{SimpleTokenizer, TextAnalyzer, Token};

    use super::VnCore;
    /// This is a function that can be used in tests and doc tests
    /// to assert a token's correctness.
    pub fn assert_token(token: &Token, position: usize, text: &str, from: usize, to: usize) {
        assert_eq!(
            token.position, position,
            "expected position {position} but {token:?}"
        );
        assert_eq!(token.text, text, "expected text {text} but {token:?}");
        assert_eq!(
            token.offset_from, from,
            "expected offset_from {from} but {token:?}"
        );
        assert_eq!(token.offset_to, to, "expected offset_to {to} but {token:?}");
    }
    #[test]
    fn test_simple_tokenizer() {
        let tokens = token_stream_helper("Ông Nguyễn Khắc Chúc đang làm việc tại Đại học Quốc gia Hà Nội. Bà Lan, vợ ông Chúc, cũng làm việc tại đây.");
        assert_eq!(tokens.len(), 21);
        assert_token(&tokens[0], 0, "Ông", 0, 4);
        assert_token(&tokens[1], 1, "Nguyễn Khắc Chúc", 5, 22);
        assert_token(&tokens[2], 2, "đang", 23, 28);
        assert_token(&tokens[3], 3, "làm việc", 29, 38);
        assert_token(&tokens[4], 4, "tại", 39, 43);
        assert_token(&tokens[5], 5, "Đại học", 44, 52);
        assert_token(&tokens[6], 6, "Quốc gia", 53, 62);
        assert_token(&tokens[7], 7, "Hà Nội", 63, 69);
        assert_token(&tokens[8], 8, ".", 69, 71);
        assert_token(&tokens[9], 9, "Bà", 72, 75);
        assert_token(&tokens[10], 10, "Lan", 76, 80);
        assert_token(&tokens[11], 11, ",", 80, 82);
        assert_token(&tokens[12], 12, "vợ", 83, 86);
        assert_token(&tokens[13], 13, "ông", 87, 91);
        assert_token(&tokens[14], 14, "Chúc", 92, 96);
        assert_token(&tokens[15], 15, ",", 96, 98);
        assert_token(&tokens[16], 16, "cũng", 99, 103);
        assert_token(&tokens[17], 17, "làm việc", 104, 113);
        assert_token(&tokens[18], 18, "tại", 114, 118);
        assert_token(&tokens[19], 19, "đây", 119, 122);
        assert_token(&tokens[20], 20, ".", 122, 124);
    }

    fn token_stream_helper(text: &str) -> Vec<Token> {
        let mut a = TextAnalyzer::from(VnCore::default());
        let mut token_stream = a.token_stream(text);
        let mut tokens: Vec<Token> = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);
        tokens
    }
}
