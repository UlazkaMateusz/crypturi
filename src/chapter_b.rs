use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tera::Context;

pub trait RegisterB {
    fn register_chapter_b(self) -> Router<AppState>;
}

impl RegisterB for Router<AppState> {
    fn register_chapter_b(self) -> Router<AppState> {
        self.route("/chapter/b/level/{level}", get(chapter_b))
    }
}

struct ChapterBContext {
    level: String,
}

impl Into<Context> for ChapterBContext {
    fn into(self) -> Context {
        let mut context = Context::new();
        context.insert("level", &self.level);

        let current_level_number = IDS
            .iter()
            .position(|id| id == &self.level)
            .map(|x| x as i32)
            .unwrap_or(-1);

        context.insert("current_level_number", &(current_level_number + 1));

        if current_level_number == 99 {
            context.insert(
                "special_button_text",
                "Let's go to chapter C");
            context.insert("next_level_url", "/chapter/c/a7f6f851-ae3a-4d47-8562-2b0e8fdb14a0");
        } else if current_level_number == -1 {
            context.insert(
                "next_level_url",
                &format!("/chapter/b/level/{}", IDS.first().unwrap()),
            );
        } else {
            let next_level_id = IDS[current_level_number as usize + 1];
            context.insert(
                "next_level_url",
                &format!("/chapter/b/level/{}", next_level_id),
            );
        }

        context
    }
}

async fn chapter_b(State(state): State<AppState>, Path(level): Path<String>) -> Html<String> {
    let context = ChapterBContext { level };

    let rendered = state
        .get_templates()
        .await
        .render("chapter_b.html", &context.into())
        .unwrap();

    Html(rendered)
}

pub const IDS: [&str; 100] = [
    "b4dff9a6-efd6-4556-b27e-1f64d7485b4e",
    "3e12a264-5f65-4f17-8691-d7b34058f61a",
    "f4b56bdc-784a-4981-bbaa-9635beed6222",
    "58aa4c8a-bf8b-4174-a49b-aa6e09bc33c5",
    "eb02a73f-8a3a-437b-9791-4c9c3e93c03f",
    "c2fbf6fa-3328-4b4c-ab8d-ab844cae308a",
    "a71e3e87-d7b5-4f01-9773-36d17ed72d2e",
    "973d5382-d23c-413d-bb19-b23326fd5a06",
    "659cc28b-763d-44bb-a73a-9cfa4d5c28e9",
    "ee1840f7-3dbf-495c-bab5-4bb1923ae10b",
    "41e40962-0ed1-41fa-898a-9d73b0ddb6c8",
    "e6d18872-d1b9-46d3-9b91-93fff37c19aa",
    "fe541c8e-fef5-4b6b-821d-85be64ffaf55",
    "fe0c16ff-adf9-4d74-9df0-da2cf8614450",
    "99b5daf7-94be-4c83-addd-827cdd2b0dd9",
    "9afe7f6a-4e96-461e-bb7f-402eeaea3fa4",
    "224c13e0-5883-4504-9219-b1f8e9d86611",
    "61e1220a-116c-4bc7-b916-20ba704bab41",
    "4e3598a0-48a5-45f3-adef-d81df303d53a",
    "f9c6a1c2-58f0-4130-b4d3-f0511fcb1a51",
    "c1a8350c-5729-4839-ad9c-75080d46013b",
    "334031b2-c857-4b09-a8ff-625c4d21cf3c",
    "fed04f0f-3e0b-4a5c-8b05-d5039b46b5d4",
    "eb0f9a96-ab5d-425c-8bc6-7fa5b11b6ab6",
    "5952b9f4-a6bf-430e-a43c-6d4fac32d601",
    "7fd79cef-f637-4d26-aa1d-a50855513272",
    "0ee02f8e-0cef-474b-9ee0-f8bbc4f3a003",
    "2a190547-9cf0-4690-aa04-cfd32d1d7ccf",
    "9c1d2ccf-0322-406e-8287-405c6a6e2c77",
    "321ee9b6-59d3-4d37-9c3b-680df1d224ba",
    "29ed1f7a-cdcd-42ce-b649-d87b2d0c8293",
    "e516f19c-ab0f-4358-b623-b20c75f5ed0a",
    "1d0fbae8-346a-4bf5-8df7-b992fb8d5107",
    "98315bbf-8385-482a-800f-678811fefad1",
    "eff81927-be1c-464b-aae9-41d72cdeb210",
    "15f5d9fa-3d7b-4506-b8cf-3db2b5c4e4d5",
    "44060570-4cf0-4dad-b8b0-deff104b1900",
    "98fb310e-ea16-471f-bc93-a89aca137217",
    "d319b260-963d-42cb-9692-e8abbf3e8a80",
    "8b7fcc01-1a47-414c-860b-0a428576b473",
    "8dac3457-b57d-4610-a1fa-e44c7cb675ef",
    "866efd1b-612d-45d1-afae-28abdd0392a9",
    "cdf9dff5-b968-4e82-b8af-c683d07a9628",
    "358262ee-b5d0-49a5-8007-ea6a704f1059",
    "aa225511-cef3-44bf-aacd-edbe85859f77",
    "767caf77-bc45-42a8-831e-66ec7ef673a0",
    "f8771ba6-d3ff-484e-a62e-60c69a7b6648",
    "174cfe5a-cdde-4ef2-9528-37f4aab5e05c",
    "728299c2-1272-49ef-84d8-4bf8a0dcb1fb",
    "7113b86f-39f6-4db7-ac71-2bc17a7e5cd6",
    "b3dc527d-8dec-4dad-aeeb-fe9c5a13db1b",
    "ee3a01c1-5a0f-4efe-8ca0-4887cf9f8928",
    "e32fe592-e387-4900-9aa9-a71a68331c6b",
    "d5979668-61ea-4b2b-a665-2de7cd9a895d",
    "eafff98d-d9cc-4802-8cdf-ceb6e475cd29",
    "6e222898-e9ef-4c9d-82c5-4ce5e0188b2d",
    "ccea8053-f185-42e2-96e1-d3d46fa2e077",
    "ca948547-b815-4ec8-bfe9-e377840a637f",
    "5197ff4d-2d50-424c-a64a-d60f669a491d",
    "71c41a70-95ad-436f-bbc0-7e36df0c66c3",
    "8e8756d2-e55b-4b71-b4a0-b033dca16abc",
    "cdf05c18-970f-4158-a65a-d8ced7bfb51b",
    "21dc3acb-338e-4fb6-b326-0d94ed39f515",
    "f8dafafd-75de-4c86-8cc0-11cda9c3da10",
    "18d2b56e-9da5-4d90-8041-d7f7c1fb4c8d",
    "b39c7fc1-1d54-4800-8b9c-a24edcd7c2ea",
    "c35bed14-9ab8-4240-9612-290b5e859ec0",
    "a89b5b34-f84d-4b28-9e96-5063d8ff672c",
    "166ff90b-bb0a-4a66-a7b2-14f0267ff585",
    "446b1ad4-fcc9-4e53-af03-ede1116d4eaa",
    "b2d8a844-081c-4457-b55a-d2268d6d04f4",
    "ca568913-32f0-484a-bd3d-05136bcb8e0e",
    "658c3fdf-37d5-4723-a7ea-ac28cafe8d2f",
    "3f0a79d5-26c4-4f43-a360-15cd1c1a431e",
    "4a521a43-92cd-4bc1-bc9e-3c6df4aca7b3",
    "0e5fe144-68a9-415a-9e9c-c2e109ebbbb0",
    "13729b7a-b1fc-45f0-b3b8-98b1dc444814",
    "c3b41752-71f5-4a88-97e7-7cfa556b1be2",
    "b9086446-ed02-423d-8af1-53263b3b6e93",
    "737576a7-79b7-45f4-b7da-2aa7d28f64bf",
    "3feb9746-b656-46b0-9e50-b01bd126905b",
    "b0664c3b-1367-4293-8f9a-256b443f7c4b",
    "00d03ea5-dbd7-4d91-92e4-ea372f5e9324",
    "01c92411-2171-4658-9273-e1377785c841",
    "3b483a21-4812-4c29-bfb6-40273f734318",
    "54331e83-a633-434f-8d21-9a2e7a71dd0a",
    "5fc84fdf-b522-49bc-a96d-3e3f87381405",
    "a3951498-6878-4d9f-8c4c-80ef1cd58641",
    "bea150ca-e4e3-4dd3-8785-8f2041236ce0",
    "ba44fecc-6b1e-4a6f-9b48-20cc475aa89a",
    "43dddcfe-5ae5-4a3e-96e0-210abcaa9d89",
    "b453296d-d8e4-4848-a61a-960b7da2012d",
    "cecba0b4-a6fa-401f-9bb4-9d3153835adc",
    "417764c3-4d46-4a57-a6ea-9e29f36910c9",
    "fe985bed-4af8-4559-a998-fea9036277ae",
    "5a492921-db10-49dd-9e5e-45d5ae6a5790",
    "a673302b-19d5-496e-acf3-69cfdfe1c8d9",
    "41eea70c-4dfe-4901-b4d9-e910f123a498",
    "8c2838b0-6ea2-4cdb-8caa-080df4d639bd",
    "17c69132-23e1-481b-be0d-7fa3c6821246",
];
