use biblatex::Bibliography;
use biblatex::ChunksExt;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Index {
    title: String,
    bib_raw: Vec<biblatex::Entry>,
}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            title: "Hello, World from Rust!".to_string(),
            bib_raw: Vec::new(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    // Iterate over bibliography entries and create a list of HTML elements
    fn view(&self) -> Html {
        // Create Bibliography object from &str using biblatex
        fn parse_bib(bib: &str) -> Bibliography {
            let bibliography = Bibliography::parse(bib).unwrap();
            bibliography
        }

        // TODO: https://stackoverflow.com/a/73410587
        // let bib_raw = async {reqwest::get("/papers.bib").await?.text().await};

        // let bib = load_bib();
        // let bibliography = parse_bib(&bib);
        // let mut entries = Vec::new();

        // // Iterate over bibliography entries and create a vector of <div> strings in html! macro
        // for x in bibliography {
        //     let authors = x.author().unwrap_or_default();
        //     let title = x.title().unwrap().format_sentence();
        //     let url = x.url().unwrap_or_default();
        //     let doi = x.doi().unwrap_or_default();

        //     let entry = html! {
        //         <div>
        //             <p>
        //                 {authors}
        //                 {title}
        //                 {url}
        //                 {doi}
        //             </p>
        //         </div>
        //     };
        //     entries.push(entry);
        // }

        // Return the vector of <div> strings in html! macro
        html! {
            <div>
                {&self.title}
                <br/>
                {"List of publications:"}
                <br/>
                // {&self.bib_raw}
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // Load bibliography from file
        }
    }
}

fn main() {
    yew::start_app::<Index>();
}
