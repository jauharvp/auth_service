use markup5ever :: {
namespace_prefix , namespace_url , ns , tendril :: SliceExt , local_name
} ; use crate :: tokenizer :: states :: {
Rawtext , Rcdata
} ; use crate :: {
expanded_name , y_name
} ; fn current_node < Handle > (open_elems : & [Handle]) -> & Handle {
open_elems . last () . expect ("no current element")
} # [doc (hidden)] impl < Handle , Sink > TreeBuilder < Handle , Sink > where Handle : Clone , Sink : TreeSink < Handle = Handle > , {
fn step (& mut self , mode : InsertionMode , token : Token) -> ProcessResult < Handle > {
self . debug_step (mode , & token) ; match mode {
Initial => match token {
CharacterTokens (NotSplit , text) => SplitWhitespace (text) , CharacterTokens (Whitespace , _) => Done , tag @ CharacterTokens (NotWhitespace , _) => self . unexpected (& tag) , last_arm_token => {
let enable_wildcards = match last_arm_token {
_ => true ,
} ; match (enable_wildcards , last_arm_token) {
(_ , token) => Reprocess (BeforeHtml , token) ,
}
}
} , BeforeHtml => match token {
CharacterTokens (NotSplit , text) => SplitWhitespace (text) , CharacterTokens (Whitespace , _) => Done , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("html") , ..
}) => {
self . create_root (tag . attrs) ; self . mode = InHtml ; Done
} , last_arm_token => {
let enable_wildcards = match last_arm_token {
crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("head") , ..
}) => false , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("body") , ..
}) => false , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("html") , ..
}) => false , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("br") , ..
}) => false , _ => true ,
} ; match (enable_wildcards , last_arm_token) {
(_ , tag) => self . unexpected (& tag) ,
}
}
} , InHtml => match token {
CharacterTokens (_ , text) => self . append_text (text) , CommentToken (text) => self . append_comment (text) , NullCharacterToken => self . unexpected (& token) , EOFToken => {
self . stop_parsing ()
} , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("body") , ..
}) => {
if self . in_scope_named (default_scope , y_name ! ("body")) {
self . check_body_end () ; self . mode = AfterBody ;
} else {
self . sink . parse_error (Borrowed ("</body> with no <body> in scope")) ;
} Done
} , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("html") , ..
}) => {
if self . in_scope_named (default_scope , y_name ! ("body")) {
self . check_body_end () ; Reprocess (AfterBody , token)
} else {
self . sink . parse_error (Borrowed ("</html> with no <body> in scope")) ; Done
}
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("base") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("basefont") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("bgsound") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("link") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("meta") , ..
}) => {
self . insert_and_pop_element_for (tag) ; DoneAckSelfClosing
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("area") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("br") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("embed") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("img") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("keygen") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("wbr") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("input") , ..
}) => {
self . reconstruct_formatting () ; self . insert_and_pop_element_for (tag) ; DoneAckSelfClosing
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("address") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("applet") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("article") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("aside") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("blockquote") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("body") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("caption") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("center") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("col") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("colgroup") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dd") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("details") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dialog") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dir") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("div") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dl") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dt") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("fieldset") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("figcaption") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("figure") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("footer") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("form") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("frame") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("frameset") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("head") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("header") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("hgroup") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("li") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("main") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("marquee") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("menu") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("nav") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("object") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("ol") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("p") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("section") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("select") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("summary") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("table") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("tbody") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("td") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("tfoot") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("th") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("thead") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("tr") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("ul") , ..
}) => {
self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h1") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h2") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h3") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h4") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h5") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h6") , ..
}) => {
if self . current_node_in (heading_tag) {
self . sink . parse_error (Borrowed ("nested heading tags")) ; self . pop () ;
} self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("pre") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("listing") , ..
}) => {
self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("plaintext") , ..
}) => {
self . insert_element_for (tag) ; ToPlaintext
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("button") , ..
}) => {
if self . in_scope_named (default_scope , y_name ! ("button")) {
self . sink . parse_error (Borrowed ("nested buttons")) ; self . generate_implied_end (cursory_implied_end) ; self . pop_until_named (y_name ! ("button")) ;
} self . reconstruct_formatting () ; self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("address") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("applet") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("article") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("aside") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("blockquote") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("caption") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("center") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("col") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("colgroup") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("details") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("dialog") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("dir") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("div") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("dl") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("fieldset") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("figcaption") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("figure") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("footer") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("form") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("frame") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("frameset") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("head") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("header") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("hgroup") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("main") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("marquee") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("menu") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("nav") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("object") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("ol") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("section") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("select") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("summary") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("table") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("tbody") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("td") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("tfoot") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("th") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("thead") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("tr") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("ul") , ..
}) => {
if ! self . in_scope_named (default_scope , tag . name . clone ()) {
self . unexpected (& tag) ;
} else {
self . generate_implied_end (cursory_implied_end) ; self . expect_to_close (tag . name) ;
} Done
} , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("p") , ..
}) => {
if ! self . in_scope_named (button_scope , y_name ! ("p")) {
self . sink . parse_error (Borrowed ("No <p> tag to close")) ; self . insert_phantom (y_name ! ("p")) ;
} self . close_p_element () ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("li") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("dd") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("dt") , ..
}) => {
let in_scope = if tag . name == y_name ! ("li") {
self . in_scope_named (list_item_scope , tag . name . clone ())
} else {
self . in_scope_named (default_scope , tag . name . clone ())
} ; if in_scope {
self . generate_implied_end_except (tag . name . clone ()) ; self . expect_to_close (tag . name) ;
} else {
self . sink . parse_error (Borrowed ("No matching tag to close")) ;
} Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("h1") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("h2") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("h3") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("h4") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("h5") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("h6") , ..
}) => {
if self . in_scope (default_scope , | n | self . elem_in (& n , heading_tag)) {
self . generate_implied_end (cursory_implied_end) ; if ! self . current_node_named (tag . name) {
self . sink . parse_error (Borrowed ("Closing wrong heading tag")) ;
} self . pop_until (heading_tag) ;
} else {
self . sink . parse_error (Borrowed ("No heading tag to close")) ;
} Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("a") , ..
}) => {
self . handle_misnested_a_tags (& tag) ; self . reconstruct_formatting () ; self . create_formatting_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("b") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("big") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("code") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("em") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("font") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("i") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("s") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("small") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("strike") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("strong") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("tt") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("u") , ..
}) => {
self . reconstruct_formatting () ; self . create_formatting_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("nobr") , ..
}) => {
self . reconstruct_formatting () ; if self . in_scope_named (default_scope , y_name ! ("nobr")) {
self . sink . parse_error (Borrowed ("Nested <>obr>")) ; self . adoption_agency (y_name ! ("nobr")) ; self . reconstruct_formatting () ;
} self . create_formatting_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("a") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("b") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("big") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("code") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("em") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("font") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("i") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("nobr") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("s") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("small") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("strike") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("strong") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("tt") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("u") , ..
}) => {
self . adoption_agency (tag . name) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("br") , ..
}) => {
self . unexpected (& tag) ; self . step (InHtml , TagToken (Tag {
kind : StartTag , attrs : vec ! () , .. tag
}))
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("param") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("source") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("track") , ..
}) => {
self . insert_and_pop_element_for (tag) ; DoneAckSelfClosing
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("hr") , ..
}) => {
self . insert_and_pop_element_for (tag) ; DoneAckSelfClosing
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("image") , ..
}) => {
self . unexpected (& tag) ; self . step (InHtml , TagToken (Tag {
name : y_name ! ("img") , .. tag
}))
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("textarea") , ..
}) => {
self . parse_raw_data (tag , Rcdata)
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("xmp") , ..
}) => {
self . reconstruct_formatting () ; self . parse_raw_data (tag , Rawtext)
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("iframe") , ..
}) => {
self . parse_raw_data (tag , Rawtext)
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("noembed") , ..
}) => {
self . parse_raw_data (tag , Rawtext)
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("rb") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("rtc") , ..
}) => {
if self . in_scope_named (default_scope , y_name ! ("ruby")) {
self . generate_implied_end (cursory_implied_end) ;
} if ! self . current_node_named (y_name ! ("ruby")) {
self . unexpected (& tag) ;
} self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("rp") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("rt") , ..
}) => {
if self . in_scope_named (default_scope , y_name ! ("ruby")) {
self . generate_implied_end_except (y_name ! ("rtc")) ;
} if ! self . current_node_named (y_name ! ("rtc")) && ! self . current_node_named (y_name ! ("ruby")) {
self . unexpected (& tag) ;
} self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("option") , ..
}) => {
if self . current_node_named (y_name ! ("option")) {
self . pop () ;
} self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("optgroup") , ..
}) => {
if self . current_node_named (y_name ! ("option")) {
self . pop () ;
} if self . current_node_named (y_name ! ("optgroup")) {
self . pop () ;
} self . insert_element_for (tag) ; Done
} , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("optgroup") , ..
}) => {
if self . open_elems . len () >= 2 && self . current_node_named (y_name ! ("option")) && self . html_elem_named (& self . open_elems [self . open_elems . len () - 2] , y_name ! ("optgroup")) {
self . pop () ;
} if self . current_node_named (y_name ! ("optgroup")) {
self . pop () ;
} else {
self . unexpected (& token) ;
} Done
} , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("option") , ..
}) => {
if self . current_node_named (y_name ! ("option")) {
self . pop () ;
} else {
self . unexpected (& token) ;
} Done
} , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("math") , ..
}) => self . enter_foreign (tag , ns ! (mathml)) , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("svg") , ..
}) => self . enter_foreign (tag , ns ! (svg)) , last_arm_token => {
let enable_wildcards = match last_arm_token {
_ => true ,
} ; match (enable_wildcards , last_arm_token) {
(true , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , ..
})) => {
self . reconstruct_formatting () ; self . insert_element_for (tag) ; Done
} , (true , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , ..
})) => {
self . process_end_tag_in_body (tag) ; Done
} , (_ , _) => panic ! ("impossible case in InHtml mode") ,
}
}
} , AfterBody => match token {
CharacterTokens (NotSplit , text) => SplitWhitespace (text) , CharacterTokens (Whitespace , _) => Done , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("html") , ..
}) => self . step (InHtml , token) , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , name : y_name ! ("html") , ..
}) => {
if self . is_fragment () {
self . unexpected (& token) ;
} else {
self . mode = AfterAfterBody ;
} Done
} , EOFToken => self . stop_parsing () , last_arm_token => {
let enable_wildcards = match last_arm_token {
_ => true ,
} ; match (enable_wildcards , last_arm_token) {
(_ , token) => {
self . unexpected (& token) ; Reprocess (InHtml , token)
} ,
}
}
} , AfterAfterBody => match token {
CharacterTokens (NotSplit , text) => SplitWhitespace (text) , CharacterTokens (Whitespace , _) => Done , crate :: tree_builder :: types :: TagToken (crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("html") , ..
}) => self . step (InHtml , token) , EOFToken => self . stop_parsing () , last_arm_token => {
let enable_wildcards = match last_arm_token {
_ => true ,
} ; match (enable_wildcards , last_arm_token) {
(_ , token) => self . unexpected (& token) ,
}
}
} , RawText => match token {
CommentToken (text) => self . append_comment (text) , CharacterTokens (_ , text) => self . append_text (text) , EOFToken => {
self . unexpected (& token) ; if self . current_node_named (y_name ! ("script")) {
let current = current_node (& self . open_elems) ; self . sink . mark_script_already_started (current) ;
} self . pop () ; Reprocess (self . orig_mode . take () . unwrap () , token)
} , last_arm_token => {
let enable_wildcards = match last_arm_token {
_ => true ,
} ; match (enable_wildcards , last_arm_token) {
(true , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , ..
})) => {
let node = self . pop () ; self . mode = self . orig_mode . take () . unwrap () ; if tag . name == y_name ! ("script") {
return Script (node) ;
} Done
} , (_ , _) => panic ! ("impossible case in Text mode") ,
}
}
} ,
}
} fn step_foreign (& mut self , token : Token) -> ProcessResult < Handle > {
match token {
NullCharacterToken => {
self . unexpected (& token) ; self . append_text ("\u{fffd}" . to_tendril ())
} , CharacterTokens (_ , text) => {
self . append_text (text)
} , CommentToken (text) => self . append_comment (text) , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("b") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("big") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("blockquote") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("body") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("br") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("center") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("code") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dd") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("div") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dl") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("dt") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("em") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("embed") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h1") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h2") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h3") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h4") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h5") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("h6") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("head") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("hr") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("i") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("img") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("li") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("listing") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("menu") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("meta") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("nobr") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("ol") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("p") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("pre") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("ruby") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("s") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("small") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("span") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("strong") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("strike") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("sub") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("sup") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("table") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("tt") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("u") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("ul") , ..
}) | crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("var") , ..
}) => self . unexpected_start_tag_in_foreign_content (tag) , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , name : y_name ! ("font") , ..
}) => {
let unexpected = tag . attrs . iter () . any (| attr | {
matches ! (attr . name . expanded () , expanded_name ! ("" , "color") | expanded_name ! ("" , "face") | expanded_name ! ("" , "size"))
}) ; if unexpected {
self . unexpected_start_tag_in_foreign_content (tag)
} else {
self . foreign_start_tag (tag)
}
} , last_arm_token => {
let enable_wildcards = match last_arm_token {
_ => true ,
} ; match (enable_wildcards , last_arm_token) {
(true , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: StartTag , ..
})) => self . foreign_start_tag (tag) , (true , crate :: tree_builder :: types :: TagToken (tag @ crate :: tokenizer :: Tag {
kind : crate :: tokenizer :: EndTag , ..
})) => {
let mut first = true ; let mut stack_idx = self . open_elems . len () - 1 ; loop {
if stack_idx == 0 {
return Done ;
} let html ; let eq ; {
let node_name = self . sink . elem_name (& self . open_elems [stack_idx]) ; html = * node_name . ns == ns ! (html) ; eq = node_name . local . eq_ignore_ascii_case (& tag . name) ;
} if ! first && html {
let mode = self . mode ; return self . step (mode , TagToken (tag)) ;
} if eq {
self . open_elems . truncate (stack_idx) ; return Done ;
} if first {
self . unexpected (& tag) ; first = false ;
} stack_idx -= 1 ;
}
} , (_ , _) => panic ! ("impossible case in foreign content") ,
}
}
}
}
}