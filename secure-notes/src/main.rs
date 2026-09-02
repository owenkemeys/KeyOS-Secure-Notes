mod theme;

use crate::fs_permissions::FileSystemPermissions;
use secure_notes_core::{
    account_input, fields_as_input, information_body, note_type_input, secret_input, tags_as_input,
    BitwardenImportPreview, BitwardenImportReviewItem as CoreImportReviewItem, NotesVault,
    SecureNote,
};
use slint_keyos_platform::{
    app_ui2,
    file_backed::JsonBacked,
    gui_server_api::navigation::{
        filepicker::{
            AllowedExtensions, AllowedLocations, Location as PickerLocation, SelectFileOptions,
        },
        lockscreen::VerifyPinOptions,
    },
    navigation::{select_file, verify_pin},
    slint::{ModelRc, SharedString, VecModel},
};
use std::{cell::RefCell, io::Read, rc::Rc};

app_ui2!("Secure Notes");

const DATABASE_FILE: &str = "secure_notes_v1.json";

struct UiState {
    vault: JsonBacked<NotesVault, FileSystemPermissions>,
    query: String,
    folder_filter: String,
    type_filter: String,
    tag_filter: String,
    sort_order: String,
    selected_id: Option<u64>,
    pending_import: Option<PendingImport>,
    notice: String,
}

struct PendingImport {
    json: String,
    path: String,
    review_items: Vec<CoreImportReviewItem>,
}

struct ImportResultRow {
    title: String,
    subtitle: String,
    status: &'static str,
}

impl UiState {
    fn new() -> Self {
        let (mut vault, restored): (JsonBacked<NotesVault, FileSystemPermissions>, bool) =
            JsonBacked::new(DATABASE_FILE, fs::Location::AppData);
        let migrated = vault.guard().migrate_legacy_items();
        let deduplicated = vault.guard().deduplicate_by_title();
        log::info!(
            "secure notes database ready: restored={}, migrated={}, deduplicated={}, notes={}",
            restored,
            migrated,
            deduplicated,
            vault.notes.len()
        );

        Self {
            vault,
            query: String::new(),
            folder_filter: "All folders".to_string(),
            type_filter: "All types".to_string(),
            tag_filter: "All tags".to_string(),
            sort_order: "A-Z".to_string(),
            selected_id: None,
            pending_import: None,
            notice: String::new(),
        }
    }

    fn active_ids(&self) -> Vec<u64> {
        self.vault.active_ids_with_filters(
            &self.query,
            &self.folder_filter,
            &self.type_filter,
            &self.tag_filter,
        )
    }

    fn archived_ids(&self) -> Vec<u64> {
        self.vault.archived_ids("")
    }
}

fn app_main(cx: AppContext, ui: AppWindow) {
    log_server::init_wait(env!("CARGO_CRATE_NAME")).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    theme::init(&ui);

    let state = Rc::new(RefCell::new(UiState::new()));
    render_active_list(&ui, &state.borrow());

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_search_changed(move |query| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.query = query.to_string();
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_filter_folder_changed(move |folder| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.folder_filter = folder.to_string();
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_filter_type_changed(move |item_type| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.type_filter = item_type.to_string();
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_filter_tag_changed(move |tag| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.tag_filter = tag.to_string();
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_sort_order_changed(move |sort_order| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.sort_order = sort_order.to_string();
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_filter_folder_index_changed(move |index| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            let options = filter_options("All folders", state.vault.folder_options());
            state.folder_filter = option_value_at(&options, index, "All folders");
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_filter_type_index_changed(move |index| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            let options = type_filter_options();
            state.type_filter = option_value_at(&options, index, "All types");
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_filter_tag_index_changed(move |index| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            let options = filter_options("All tags", state.vault.tag_options());
            state.tag_filter = option_value_at(&options, index, "All tags");
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_sort_order_index_changed(move |index| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            let options = sort_options();
            state.sort_order = option_value_at(&options, index, "A-Z");
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_row_selected(move |id| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let id = id.max(0) as u64;
            if !pin_allows_viewing(&state, id) {
                let mut state = state.borrow_mut();
                state.selected_id = None;
                state.notice = "PIN verification was canceled or failed.".to_string();
                render_active_list(&ui, &state);
                return;
            }

            let mut state = state.borrow_mut();
            state.selected_id = Some(id);
            state.notice.clear();
            render_detail(&ui, &state, false);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_archive_row_selected(move |id| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let id = id.max(0) as u64;
            if !pin_allows_viewing(&state, id) {
                let mut state = state.borrow_mut();
                state.selected_id = None;
                state.notice = "PIN verification was canceled or failed.".to_string();
                render_archive_list(&ui, &state);
                return;
            }

            let mut state = state.borrow_mut();
            state.selected_id = Some(id);
            state.notice.clear();
            render_detail(&ui, &state, true);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_new_note_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.selected_id = None;
            state.notice.clear();
            render_edit(&ui, &state, None);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_view_archive_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.selected_id = None;
            state.notice.clear();
            render_archive_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_back_to_list_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.selected_id = None;
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_back_to_archive_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.selected_id = None;
            state.notice.clear();
            render_archive_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_edit_back_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let state = state.borrow();
            if state.selected_id.is_some() {
                render_detail(&ui, &state, false);
            } else {
                render_active_list(&ui, &state);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_edit_note_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let state = state.borrow();
            let note = state.selected_id.and_then(|id| state.vault.note_by_id(id));
            render_edit(&ui, &state, note);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_save_note_requested(
            move |title,
                  item_type,
                  account,
                  secret,
                  fields,
                  folder,
                  tags,
                  favorite,
                  pin_required| {
                let Some(ui) = ui_weak.upgrade() else {
                    return;
                };
                let mut state = state.borrow_mut();
                let current_selected_id = state.selected_id;
                if !validate_edit_form(
                    &ui,
                    &state.vault,
                    current_selected_id,
                    title.as_str(),
                    item_type.as_str(),
                    secret.as_str(),
                ) {
                    return;
                }
                let selected_id = {
                    let mut vault = state.vault.guard();
                    if let Some(id) = current_selected_id {
                        vault.update_manual_item_full(
                            id,
                            title.to_string(),
                            item_type.to_string(),
                            account.to_string(),
                            secret.to_string(),
                            tags.to_string(),
                            fields.to_string(),
                            folder.to_string(),
                            favorite,
                            pin_required,
                        );
                        id
                    } else {
                        vault.add_manual_item_full(
                            title.to_string(),
                            item_type.to_string(),
                            account.to_string(),
                            secret.to_string(),
                            tags.to_string(),
                            fields.to_string(),
                            folder.to_string(),
                            favorite,
                            pin_required,
                        )
                    }
                };

                state.selected_id = Some(selected_id);
                state.notice.clear();
                render_detail(&ui, &state, false);
            },
        );
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_custom_field_value_changed(move |index, value| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let fields = update_field_value(
                ui.get_edit_fields().as_str(),
                index.max(0) as usize,
                value.as_str(),
            );
            ui.set_edit_fields(SharedString::from(fields.clone()));
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_custom_field_rename_requested(move |index, label, hidden| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let fields = rename_field_label(
                ui.get_edit_fields().as_str(),
                index.max(0) as usize,
                label.as_str(),
                hidden,
            );
            ui.set_edit_fields(SharedString::from(fields.clone()));
            ui.set_edit_field_items(custom_field_model(&fields));
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_custom_field_delete_requested(move |index| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let fields = delete_field(ui.get_edit_fields().as_str(), index.max(0) as usize);
            ui.set_edit_fields(SharedString::from(fields.clone()));
            ui.set_edit_field_items(custom_field_model(&fields));
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_add_field_requested(move |kind, label, value| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let fields = append_custom_field(
                ui.get_edit_fields().as_str(),
                kind.as_str(),
                label.as_str(),
                value.as_str(),
            );
            ui.set_edit_fields(SharedString::from(fields.clone()));
            ui.set_edit_field_items(custom_field_model(&fields));
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_new_folder_created(move |folder| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let folder = folder.as_str().trim().to_string();
            if folder.is_empty() {
                ui.set_new_folder_error(SharedString::from(
                    "Missing field, please add a Folder name",
                ));
                return;
            }

            let state = state.borrow();
            let folder_options = folder_assign_options(state.vault.folder_options(), &folder);
            let folder_index = option_index(&folder_options, &folder);
            ui.set_edit_folder(SharedString::from(folder));
            ui.set_edit_folder_options(string_model(&folder_options));
            ui.set_edit_folder_index(folder_index);
            ui.set_edit_folder_committed_index(folder_index);
            ui.set_new_folder_name(SharedString::from(""));
            ui.set_new_folder_error(SharedString::from(""));
            ui.set_new_folder_open(false);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_import_file_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.selected_id = None;
            state.pending_import = None;
            state.notice.clear();
            render_import_intro(&ui);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        let fs = cx.fs.clone();
        ui.on_choose_import_file_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };

            let result = pick_and_read_import_file(&fs);
            let mut state = state.borrow_mut();
            match result {
                Ok((json, path)) => match (
                    state.vault.preview_bitwarden_json(json.as_str()),
                    state.vault.review_bitwarden_json(json.as_str()),
                ) {
                    (Ok(preview), Ok(review_items)) => {
                        state.pending_import = Some(PendingImport {
                            json,
                            path: path.clone(),
                            review_items: review_items.clone(),
                        });
                        state.notice.clear();
                        render_import_preview(&ui, &path, &preview, &review_items);
                    }
                    (Err(error), _) | (_, Err(error)) => {
                        state.pending_import = None;
                        render_import_error(&ui, &path, &error.to_string());
                    }
                },
                Err(error) => {
                    state.pending_import = None;
                    if error == "No file selected." {
                        state.notice.clear();
                        render_import_intro(&ui);
                    } else {
                        render_import_error(&ui, "", &error);
                    }
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_import_review_toggled(move |source_index, selected| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            if let Some(pending) = state.pending_import.as_mut() {
                for item in &mut pending.review_items {
                    if item.source_index == source_index.max(0) as usize {
                        item.selected = selected;
                    }
                }
                ui.set_import_review_items(import_review_model(&pending.review_items));
                ui.set_import_can_continue(pending.review_items.iter().any(|item| item.selected));
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_continue_import_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            let Some(pending) = state.pending_import.take() else {
                render_import_error(&ui, "", "Choose a file before continuing.");
                return;
            };
            let selected_source_indices = pending
                .review_items
                .iter()
                .filter(|item| item.selected)
                .map(|item| item.source_index)
                .collect::<Vec<_>>();
            let selected_rows = pending
                .review_items
                .iter()
                .filter(|item| item.selected)
                .cloned()
                .collect::<Vec<_>>();
            let skipped_rows = pending
                .review_items
                .iter()
                .filter(|item| !item.selected)
                .cloned()
                .collect::<Vec<_>>();
            let result = {
                let mut vault = state.vault.guard();
                vault
                    .import_bitwarden_json_selected(pending.json.as_str(), &selected_source_indices)
                    .map_err(|error| error.to_string())
            };

            match result {
                Ok(count) => {
                    log::info!(
                        "import completed: expected={}, imported={}, path={}",
                        selected_source_indices.len(),
                        count,
                        pending.path
                    );
                    state.selected_id = None;
                    state.query.clear();
                    state.notice.clear();
                    let imported_rows = selected_rows
                        .iter()
                        .take(count)
                        .map(|item| ImportResultRow {
                            title: item.title.clone(),
                            subtitle: item.subtitle.clone(),
                            status: "Imported",
                        })
                        .collect::<Vec<_>>();
                    let failed_rows = selected_rows
                        .iter()
                        .skip(count)
                        .map(|item| ImportResultRow {
                            title: item.title.clone(),
                            subtitle: "Could not be converted to a secure note.".to_string(),
                            status: "Failed",
                        })
                        .collect::<Vec<_>>();
                    let skipped_rows = skipped_rows
                        .iter()
                        .map(|item| ImportResultRow {
                            title: item.title.clone(),
                            subtitle: if item.similar_to.is_empty() {
                                item.subtitle.clone()
                            } else {
                                format!("Similar to: \"{}\"", item.similar_to)
                            },
                            status: "Skipped by user",
                        })
                        .collect::<Vec<_>>();
                    render_import_result(&ui, count, &imported_rows, &skipped_rows, &failed_rows);
                }
                Err(error) => {
                    render_import_error(&ui, &pending.path, &error);
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_import_back_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.pending_import = None;
            state.notice.clear();
            render_import_intro(&ui);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_cancel_import_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            state.pending_import = None;
            state.notice.clear();
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_archive_note_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            if let Some(id) = state.selected_id.take() {
                let title = state
                    .vault
                    .note_by_id(id)
                    .map(|note| note.title.clone())
                    .unwrap_or_default();
                let archived = { state.vault.guard().archive_note(id) };
                if archived {
                    state.notice = format!("Archived {}.", title);
                    log::info!("secure note archived: id={id}");
                }
            }
            render_active_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_restore_note_requested(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            if let Some(id) = state.selected_id.take() {
                let title = state
                    .vault
                    .note_by_id(id)
                    .map(|note| note.title.clone())
                    .unwrap_or_default();
                let restored = { state.vault.guard().restore_note(id) };
                if restored {
                    state.notice = format!("Restored {}.", title);
                    log::info!("secure note restored: id={id}");
                }
            }
            render_archive_list(&ui, &state);
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_delete_note_confirmed(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            let was_archive_view = ui.get_mode() == 5;
            let mut state = state.borrow_mut();
            if let Some(id) = state.selected_id.take() {
                let deleted_title = { state.vault.guard().delete_note(id).map(|note| note.title) };
                if let Some(title) = deleted_title {
                    state.notice = format!("Deleted {}.", title);
                    log::info!("secure note deleted: id={id}");
                }
            }
            if was_archive_view {
                render_archive_list(&ui, &state);
            } else {
                render_active_list(&ui, &state);
            }
        });
    }

    ui.run().expect("UI running");
}

fn pin_allows_viewing(state: &Rc<RefCell<UiState>>, id: u64) -> bool {
    let pin_required = {
        let state = state.borrow();
        let Some(note) = state.vault.note_by_id(id) else {
            return false;
        };
        note.pin_required
    };

    if !pin_required {
        return true;
    }

    match verify_pin::<gui_permissions::GuiPermissions>(VerifyPinOptions {
        title: Some("Restricted Note".to_string()),
        want_security_words: false,
    }) {
        Ok(result) => result.success,
        Err(error) => {
            log::error!("pin verification failed: {error:?}");
            false
        }
    }
}

fn render_active_list(ui: &AppWindow, state: &UiState) {
    let mut ids = state.active_ids();
    sort_ids(&state.vault, &mut ids, &state.sort_order);
    let folder_options = filter_options("All folders", state.vault.folder_options());
    let type_options = type_filter_options();
    let tag_options = filter_options("All tags", state.vault.tag_options());
    let sort_options = sort_options();
    ui.set_mode(0);
    ui.set_header_title(SharedString::from("Secure Notes"));
    ui.set_search_text(SharedString::from(state.query.clone()));
    ui.set_filter_folder_options(string_model(&folder_options));
    ui.set_filter_type_options(string_model(&type_options));
    ui.set_filter_tag_options(string_model(&tag_options));
    ui.set_sort_order_options(string_model(&sort_options));
    ui.set_filter_folder(SharedString::from(state.folder_filter.clone()));
    ui.set_filter_type(SharedString::from(state.type_filter.clone()));
    ui.set_filter_tag(SharedString::from(state.tag_filter.clone()));
    ui.set_sort_order(SharedString::from(state.sort_order.clone()));
    ui.set_filter_folder_index(option_index(&folder_options, &state.folder_filter));
    ui.set_filter_type_index(option_index(&type_options, &state.type_filter));
    ui.set_filter_tag_index(option_index(&tag_options, &state.tag_filter));
    ui.set_sort_order_index(option_index(&sort_options, &state.sort_order));
    ui.set_list_items(model_for_ids(&state.vault, &ids));
    ui.set_status_text(SharedString::from(list_status(
        &state.notice,
        ids.len(),
        "note",
        state.query.trim(),
        &state.folder_filter,
        &state.type_filter,
        &state.tag_filter,
    )));
}

fn render_archive_list(ui: &AppWindow, state: &UiState) {
    let ids = state.archived_ids();
    ui.set_mode(4);
    ui.set_header_title(SharedString::from("Archive"));
    ui.set_archive_items(model_for_ids(&state.vault, &ids));
    ui.set_status_text(SharedString::from(list_status(
        &state.notice,
        ids.len(),
        "archived note",
        "",
        "All folders",
        "All types",
        "All tags",
    )));
}

fn render_detail(ui: &AppWindow, state: &UiState, archived: bool) {
    let Some(note) = state.selected_id.and_then(|id| state.vault.note_by_id(id)) else {
        if archived {
            render_archive_list(ui, state);
        } else {
            render_active_list(ui, state);
        }
        return;
    };

    ui.set_mode(if archived { 5 } else { 1 });
    ui.set_header_title(SharedString::from(if archived {
        "Archived Item"
    } else {
        "Details"
    }));
    ui.set_detail_title(SharedString::from(note.title.clone()));
    ui.set_detail_type(SharedString::from(note.item_type.label()));
    ui.set_detail_tags(SharedString::from(tag_line(&note.tags)));
    ui.set_detail_updated(SharedString::from(format!(
        "Updated: {}",
        note.updated_label
    )));
    ui.set_detail_body(SharedString::from(information_body(note)));
    ui.set_detail_field_items(custom_field_model(&fields_as_input(&note.fields)));
    ui.set_detail_pin_required(note.pin_required);
    ui.set_detail_archived(note.archived);
    ui.set_delete_title(SharedString::from("Delete Note"));
    ui.set_delete_body(SharedString::from(format!(
        "Are you sure you want to delete \"{}\"? This cannot be undone.",
        note.title
    )));
}

fn render_edit(ui: &AppWindow, state: &UiState, note: Option<&SecureNote>) {
    ui.set_mode(2);
    ui.set_header_title(SharedString::from(if note.is_some() {
        "Edit Item"
    } else {
        "Create New Item"
    }));
    ui.set_edit_header(SharedString::from(""));
    ui.set_edit_title(SharedString::from(
        note.map(|note| note.title.clone()).unwrap_or_default(),
    ));
    ui.set_edit_type(SharedString::from(
        note.map(note_type_input)
            .unwrap_or_else(|| "Secure Note".to_string()),
    ));
    ui.set_edit_type_index(note.map(note_type_index).unwrap_or(1));
    ui.set_edit_existing(note.is_some());
    ui.set_edit_label_error(SharedString::from(""));
    ui.set_edit_secret_error(SharedString::from(""));
    ui.set_edit_account(SharedString::from(
        note.map(account_input).unwrap_or_default(),
    ));
    ui.set_edit_secret(SharedString::from(
        note.map(secret_input).unwrap_or_default(),
    ));
    let fields = note
        .map(|note| fields_as_input(&note.fields))
        .unwrap_or_default();
    ui.set_edit_fields(SharedString::from(fields.clone()));
    ui.set_edit_field_items(custom_field_model(&fields));
    let edit_folder = note.map(|note| note.folder.clone()).unwrap_or_default();
    let folder_options = folder_assign_options(state.vault.folder_options(), &edit_folder);
    ui.set_edit_folder(SharedString::from(edit_folder.clone()));
    ui.set_edit_folder_options(string_model(&folder_options));
    let folder_index = option_index(
        &folder_options,
        if edit_folder.trim().is_empty() {
            "None"
        } else {
            edit_folder.as_str()
        },
    );
    ui.set_edit_folder_index(folder_index);
    ui.set_edit_folder_committed_index(folder_index);
    ui.set_new_folder_open(false);
    ui.set_new_folder_name(SharedString::from(""));
    ui.set_new_folder_error(SharedString::from(""));
    ui.set_edit_tags(SharedString::from(
        note.map(|note| tags_as_input(&note.tags))
            .unwrap_or_default(),
    ));
    ui.set_edit_favorite(note.map(|note| note.favorite).unwrap_or(false));
    ui.set_edit_pin_required(note.map(|note| note.pin_required).unwrap_or(false));
}

fn render_import_intro(ui: &AppWindow) {
    ui.set_mode(3);
    ui.set_header_title(SharedString::from("Import Item"));
    ui.set_import_stage(0);
    ui.set_import_file_label(SharedString::from(""));
    ui.set_import_summary(SharedString::from(""));
    ui.set_import_review_items(ModelRc::new(VecModel::from(
        Vec::<ImportReviewListItem>::new(),
    )));
    ui.set_import_result_items(ModelRc::new(VecModel::from(
        Vec::<ImportResultListItem>::new(),
    )));
    ui.set_import_can_continue(false);
}

fn render_import_preview(
    ui: &AppWindow,
    path: &str,
    preview: &BitwardenImportPreview,
    review_items: &[CoreImportReviewItem],
) {
    ui.set_mode(3);
    ui.set_header_title(SharedString::from("Import"));
    ui.set_import_stage(1);
    ui.set_import_file_label(SharedString::from(path));
    ui.set_import_summary(SharedString::from(import_preview_text(preview)));
    ui.set_import_review_items(import_review_model(review_items));
    ui.set_import_result_items(ModelRc::new(VecModel::from(
        Vec::<ImportResultListItem>::new(),
    )));
    ui.set_import_can_continue(review_items.iter().any(|item| item.selected));
}

fn render_import_error(ui: &AppWindow, path: &str, message: &str) {
    ui.set_mode(3);
    ui.set_header_title(SharedString::from("Import"));
    ui.set_import_stage(1);
    ui.set_import_file_label(SharedString::from(if path.is_empty() {
        "No file selected."
    } else {
        path
    }));
    ui.set_import_summary(SharedString::from(format!(
        "Cannot import this file.\n\n{message}"
    )));
    ui.set_import_review_items(ModelRc::new(VecModel::from(
        Vec::<ImportReviewListItem>::new(),
    )));
    ui.set_import_result_items(ModelRc::new(VecModel::from(
        Vec::<ImportResultListItem>::new(),
    )));
    ui.set_import_can_continue(false);
}

fn render_import_result(
    ui: &AppWindow,
    count: usize,
    imported_rows: &[ImportResultRow],
    skipped_rows: &[ImportResultRow],
    failed_rows: &[ImportResultRow],
) {
    let mut rows = Vec::new();
    rows.extend(imported_rows.iter().map(import_result_item));
    rows.extend(skipped_rows.iter().map(import_result_item));
    rows.extend(failed_rows.iter().map(import_result_item));
    ui.set_mode(3);
    ui.set_header_title(SharedString::from("Import Complete"));
    ui.set_import_stage(2);
    ui.set_import_file_label(SharedString::from(""));
    ui.set_import_summary(SharedString::from(format!(
        "{} item{} successfully imported.\n{} skipped, {} failed.",
        count,
        plural(count),
        skipped_rows.len(),
        failed_rows.len()
    )));
    ui.set_import_review_items(ModelRc::new(VecModel::from(
        Vec::<ImportReviewListItem>::new(),
    )));
    ui.set_import_result_items(ModelRc::new(VecModel::from(rows)));
    ui.set_import_can_continue(false);
}

fn note_type_index(note: &SecureNote) -> i32 {
    match note_type_input(note).as_str() {
        "Login" => 0,
        "Secure Note" => 1,
        "Payment Card" => 2,
        "Identity" => 3,
        "SSH Key" => 4,
        _ => 1,
    }
}

fn import_preview_text(preview: &BitwardenImportPreview) -> String {
    let mut lines = vec![
        format!(
            "{} item{} found.",
            preview.total_items,
            plural(preview.total_items)
        ),
        format!(
            "{} item{} can be imported.",
            preview.importable_items,
            plural(preview.importable_items)
        ),
        String::new(),
        format!("Login: {}", preview.login_items),
        format!("Secure Note: {}", preview.secure_note_items),
        format!("Payment Card: {}", preview.card_items),
        format!("Identity: {}", preview.identity_items),
        format!("SSH Key: {}", preview.ssh_key_items),
    ];

    if preview.skipped_items > 0 {
        lines.push(String::new());
        lines.push(format!(
            "{} item{} will be skipped because no readable details were found.",
            preview.skipped_items,
            plural(preview.skipped_items)
        ));
    }

    lines.push(String::new());
    if preview.similar_items == 0 {
        lines.push("No similar existing entries were found.".to_string());
    } else {
        lines.push(format!(
            "{} item{} look similar to existing entries and are off by default.",
            preview.similar_items,
            plural(preview.similar_items)
        ));
        if !preview.similar_titles.is_empty() {
            lines.push(format!("Similar: {}", preview.similar_titles.join(", ")));
        }
    }

    lines.join("\n")
}

fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn model_for_ids(vault: &NotesVault, ids: &[u64]) -> ModelRc<NoteListItem> {
    let rows = ids
        .iter()
        .filter_map(|id| vault.summary(*id))
        .map(|summary| NoteListItem {
            id: summary.id as i32,
            title: SharedString::from(summary.title),
            subtitle: SharedString::from(summary.subtitle),
            pin_required: summary.pin_required,
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(rows))
}

fn string_model(values: &[String]) -> ModelRc<SharedString> {
    ModelRc::new(VecModel::from(
        values
            .iter()
            .cloned()
            .map(SharedString::from)
            .collect::<Vec<_>>(),
    ))
}

fn filter_options(all_label: &str, mut options: Vec<String>) -> Vec<String> {
    options.retain(|option| !option.trim().is_empty());
    options.insert(0, all_label.to_string());
    options
}

fn folder_assign_options(mut options: Vec<String>, current_folder: &str) -> Vec<String> {
    options.retain(|option| {
        let option = option.trim();
        !option.is_empty()
            && !option.eq_ignore_ascii_case("None")
            && !option.eq_ignore_ascii_case("New Folder...")
    });
    let current_folder = current_folder.trim();
    if !current_folder.is_empty()
        && !options
            .iter()
            .any(|option| option.eq_ignore_ascii_case(current_folder))
    {
        options.push(current_folder.to_string());
    }
    options.sort_by_key(|option| option.to_lowercase());
    options.insert(0, "None".to_string());
    options.insert(0, "New Folder...".to_string());
    options.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    options
}

fn option_index(options: &[String], selected: &str) -> i32 {
    options
        .iter()
        .position(|option| option.eq_ignore_ascii_case(selected))
        .unwrap_or(0) as i32
}

fn option_value_at(options: &[String], index: i32, fallback: &str) -> String {
    options
        .get(index.max(0) as usize)
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

fn type_filter_options() -> Vec<String> {
    vec![
        "All types".to_string(),
        "Login".to_string(),
        "Secure Note".to_string(),
        "Payment Card".to_string(),
        "Identity".to_string(),
        "SSH Key".to_string(),
    ]
}

fn sort_options() -> Vec<String> {
    vec![
        "A-Z".to_string(),
        "Z-A".to_string(),
        "Last Updated".to_string(),
        "Recently Added".to_string(),
        "Oldest Added".to_string(),
        "Type".to_string(),
        "Folder".to_string(),
        "Favorites First".to_string(),
    ]
}

fn sort_ids(vault: &NotesVault, ids: &mut [u64], sort_order: &str) {
    fn title_key(note: Option<&SecureNote>) -> String {
        note.map(|note| note.title.to_ascii_lowercase())
            .unwrap_or_default()
    }

    ids.sort_by(|left, right| {
        let left_note = vault.note_by_id(*left);
        let right_note = vault.note_by_id(*right);
        match sort_order {
            "Z-A" => title_key(right_note).cmp(&title_key(left_note)),
            "Last Updated" => right_note
                .map(|note| note.updated_rank)
                .unwrap_or(0)
                .cmp(&left_note.map(|note| note.updated_rank).unwrap_or(0))
                .then_with(|| title_key(left_note).cmp(&title_key(right_note))),
            "Recently Added" => right
                .cmp(left)
                .then_with(|| title_key(left_note).cmp(&title_key(right_note))),
            "Oldest Added" => left
                .cmp(right)
                .then_with(|| title_key(left_note).cmp(&title_key(right_note))),
            "Type" => left_note
                .map(|note| note.item_type.label().to_ascii_lowercase())
                .unwrap_or_default()
                .cmp(
                    &right_note
                        .map(|note| note.item_type.label().to_ascii_lowercase())
                        .unwrap_or_default(),
                )
                .then_with(|| title_key(left_note).cmp(&title_key(right_note))),
            "Folder" => left_note
                .map(|note| note.folder.to_ascii_lowercase())
                .unwrap_or_default()
                .cmp(
                    &right_note
                        .map(|note| note.folder.to_ascii_lowercase())
                        .unwrap_or_default(),
                )
                .then_with(|| title_key(left_note).cmp(&title_key(right_note))),
            "Favorites First" => right_note
                .map(|note| note.favorite)
                .unwrap_or(false)
                .cmp(&left_note.map(|note| note.favorite).unwrap_or(false))
                .then_with(|| title_key(left_note).cmp(&title_key(right_note))),
            _ => title_key(left_note).cmp(&title_key(right_note)),
        }
    });
}

fn custom_field_model(input: &str) -> ModelRc<CustomFieldListItem> {
    let rows = input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (kind, rest) = custom_field_kind_and_rest(line);
            let (label, value) = rest.split_once(':').unwrap_or((rest, ""));
            let label = label.trim();
            if label.is_empty() {
                return None;
            }
            let value = if kind == "Boolean" {
                if boolean_field_is_true(value.trim()) {
                    "true"
                } else {
                    "false"
                }
            } else {
                value.trim()
            };
            Some(CustomFieldListItem {
                index: index as i32,
                label: SharedString::from(label),
                value: SharedString::from(value),
                kind: SharedString::from(kind),
            })
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(rows))
}

fn import_review_model(items: &[CoreImportReviewItem]) -> ModelRc<ImportReviewListItem> {
    let rows = items
        .iter()
        .map(|item| ImportReviewListItem {
            source_index: item.source_index as i32,
            title: SharedString::from(item.title.clone()),
            subtitle: SharedString::from(item.subtitle.clone()),
            similar_to: SharedString::from(item.similar_to.clone()),
            selected: item.selected,
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(rows))
}

fn import_result_item(row: &ImportResultRow) -> ImportResultListItem {
    ImportResultListItem {
        title: SharedString::from(row.title.clone()),
        subtitle: SharedString::from(row.subtitle.clone()),
        status: SharedString::from(row.status),
    }
}

fn validate_edit_form(
    ui: &AppWindow,
    vault: &NotesVault,
    selected_id: Option<u64>,
    title: &str,
    item_type: &str,
    secret: &str,
) -> bool {
    let mut valid = true;
    let title = title.trim();
    let label_error =
        if title.is_empty() {
            valid = false;
            "Missing field, please add a Label"
        } else if vault.notes.iter().any(|note| {
            Some(note.id) != selected_id && note.title.trim().eq_ignore_ascii_case(title)
        }) {
            valid = false;
            "Label is already in use"
        } else {
            ""
        };

    let secret_label = match item_type {
        "Login" => "Password",
        "Payment Card" => "Card Number",
        "SSH Key" => "Private Key",
        _ => "Private Details",
    };
    let secret_error = if secret.trim().is_empty() {
        valid = false;
        match secret_label {
            "Password" => "Missing field, please add a Password",
            "Card Number" => "Missing field, please add a Card Number",
            "Private Key" => "Missing field, please add a Private Key",
            _ => "Missing field, please add Private Details",
        }
    } else {
        ""
    };

    ui.set_edit_label_error(SharedString::from(label_error));
    ui.set_edit_secret_error(SharedString::from(secret_error));
    valid
}

fn custom_field_kind_and_rest(line: &str) -> (&'static str, &str) {
    if let Some(rest) = line.strip_prefix("[Hidden]") {
        ("Hidden", rest.trim())
    } else if let Some(rest) = line.strip_prefix("[Boolean]") {
        ("Boolean", rest.trim())
    } else {
        ("Text", line)
    }
}

fn custom_field_prefix(kind: &str) -> &'static str {
    match kind {
        "Hidden" => "[Hidden] ",
        "Boolean" => "[Boolean] ",
        _ => "",
    }
}

fn boolean_field_is_true(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "true" | "1" | "yes" | "on"
    )
}

fn append_custom_field(input: &str, kind: &str, label: &str, value: &str) -> String {
    let mut lines = input
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    lines.push(format!(
        "{}{}: {}",
        custom_field_prefix(kind),
        label.trim(),
        value.trim()
    ));
    lines.join("\n")
}

fn update_field_value(input: &str, index: usize, value: &str) -> String {
    input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            if line_index != index {
                return line.to_string();
            }
            let (kind, rest) = custom_field_kind_and_rest(line.trim());
            let label = rest
                .split_once(':')
                .map(|(label, _)| label)
                .unwrap_or(rest)
                .trim();
            format!("{}{}: {}", custom_field_prefix(kind), label, value.trim())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn rename_field_label(input: &str, index: usize, label: &str, hidden: bool) -> String {
    let label = label.trim();
    input
        .lines()
        .enumerate()
        .filter_map(|(line_index, line)| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            if line_index != index {
                return Some(line.to_string());
            }
            let (kind, rest) = custom_field_kind_and_rest(line);
            let kind = if kind == "Boolean" {
                "Boolean"
            } else if hidden {
                "Hidden"
            } else {
                "Text"
            };
            let value = rest
                .split_once(':')
                .map(|(_, value)| value.trim())
                .unwrap_or("");
            Some(format!("{}{}: {}", custom_field_prefix(kind), label, value))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn delete_field(input: &str, index: usize) -> String {
    input
        .lines()
        .enumerate()
        .filter_map(|(line_index, line)| {
            let line = line.trim_end();
            if line.trim().is_empty() || line_index == index {
                None
            } else {
                Some(line.to_string())
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn list_status(
    notice: &str,
    count: usize,
    noun: &str,
    query: &str,
    folder: &str,
    item_type: &str,
    tag: &str,
) -> String {
    if !notice.trim().is_empty() {
        return notice.to_string();
    }

    let plural = if count == 1 { "" } else { "s" };
    let mut filters = Vec::new();
    if !query.is_empty() {
        filters.push(format!("Search: {query}"));
    }
    if !is_all_filter(folder, "All folders") {
        filters.push(format!("Folder: {folder}"));
    }
    if !is_all_filter(item_type, "All types") {
        filters.push(format!("Type: {item_type}"));
    }
    if !is_all_filter(tag, "All tags") {
        filters.push(format!("Tag: {tag}"));
    }

    if filters.is_empty() {
        format!("{count} {noun}{plural}")
    } else {
        format!("{} - Found {count} {noun}{plural}", filters.join(" - "))
    }
}

fn is_all_filter(value: &str, all_label: &str) -> bool {
    value.trim().is_empty() || value.trim().eq_ignore_ascii_case(all_label)
}

fn tag_line(tags: &[String]) -> String {
    if tags.is_empty() {
        String::new()
    } else {
        tags.iter()
            .map(|tag| format!("#{tag}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn pick_and_read_import_file(fs: &FileSystem) -> Result<(String, String), String> {
    let options = SelectFileOptions::default()
        .with_start_location(PickerLocation::Internal)
        .with_allowed_locations(AllowedLocations::specific([
            PickerLocation::Internal,
            PickerLocation::Airlock,
            PickerLocation::External,
        ]))
        .with_allowed_extensions(AllowedExtensions::specific(["json", "JSON"]))
        .with_hidden_allowed(false)
        .with_dirs_allowed(true);

    let Some(result) = select_file::<gui_permissions::GuiPermissions>(options)
        .map_err(|error| format!("File picker failed: {error:?}"))?
    else {
        return Err("No file selected.".to_string());
    };

    let Some((path, picker_location)) = result.files().first() else {
        return Err("No file selected.".to_string());
    };

    let location = picker_location_to_fs(*picker_location);
    let mut file = fs
        .open_file(path.clone(), location, fs::OpenFlags::READ_ONLY)
        .map_err(|error| format!("Could not open {path}: {error:?}"))?;

    let metadata = file
        .metadata()
        .map_err(|error| format!("Could not inspect {path}: {error:?}"))?;
    if metadata.is_dir {
        return Err("Choose a JSON export file, not a folder.".to_string());
    }
    if metadata.size > 2 * 1024 * 1024 {
        return Err("Import file is too large for this prototype.".to_string());
    }

    let mut bytes = Vec::with_capacity(metadata.size as usize);
    file.read_to_end(&mut bytes)
        .map_err(|error| format!("Could not read {path}: {error}"))?;
    let contents =
        String::from_utf8(bytes).map_err(|_| "Import file must be UTF-8 JSON.".to_string())?;

    Ok((contents, path.clone()))
}

fn picker_location_to_fs(location: PickerLocation) -> fs::Location {
    match location {
        PickerLocation::Internal => fs::Location::User,
        PickerLocation::Airlock => fs::Location::Airlock,
        PickerLocation::External => fs::Location::Usb,
    }
}
