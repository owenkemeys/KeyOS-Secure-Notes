#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(from = "i64", into = "i64")]
pub enum BitwardenItemType {
    Login,
    SecureNote,
    Card,
    Identity,
    SshKey,
}

impl Default for BitwardenItemType {
    fn default() -> Self {
        Self::SecureNote
    }
}

impl From<i64> for BitwardenItemType {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Login,
            3 => Self::Card,
            4 => Self::Identity,
            5 => Self::SshKey,
            _ => Self::SecureNote,
        }
    }
}

impl From<BitwardenItemType> for i64 {
    fn from(value: BitwardenItemType) -> Self {
        match value {
            BitwardenItemType::Login => 1,
            BitwardenItemType::SecureNote => 2,
            BitwardenItemType::Card => 3,
            BitwardenItemType::Identity => 4,
            BitwardenItemType::SshKey => 5,
        }
    }
}

impl BitwardenItemType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Login => "Login",
            Self::SecureNote => "Secure Note",
            Self::Card => "Payment Card",
            Self::Identity => "Identity",
            Self::SshKey => "SSH Key",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BitwardenFolder {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitwardenLoginUri {
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default, rename = "match")]
    pub match_type: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BitwardenLogin {
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub totp: Option<String>,
    #[serde(default)]
    pub uris: Vec<BitwardenLoginUri>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BitwardenSecureNote {
    #[serde(default, rename = "type")]
    pub note_type: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitwardenCard {
    #[serde(default)]
    pub cardholder_name: Option<String>,
    #[serde(default)]
    pub brand: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub exp_month: Option<String>,
    #[serde(default)]
    pub exp_year: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitwardenIdentity {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub middle_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub ssn: Option<String>,
    #[serde(default)]
    pub passport_number: Option<String>,
    #[serde(default)]
    pub license_number: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub address1: Option<String>,
    #[serde(default)]
    pub address2: Option<String>,
    #[serde(default)]
    pub address3: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BitwardenField {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default, rename = "type")]
    pub field_type: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SecureNote {
    pub id: u64,
    #[serde(alias = "name")]
    pub title: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub folder: String,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub updated_label: String,
    #[serde(default)]
    pub updated_rank: u64,
    #[serde(default)]
    pub item_type: BitwardenItemType,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub login: BitwardenLogin,
    #[serde(default)]
    pub secure_note: BitwardenSecureNote,
    #[serde(default)]
    pub card: BitwardenCard,
    #[serde(default)]
    pub identity: BitwardenIdentity,
    #[serde(default, rename = "sshKey")]
    pub ssh_key: serde_json::Value,
    #[serde(default)]
    pub fields: Vec<BitwardenField>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub pin_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteSummary {
    pub id: u64,
    pub title: String,
    pub subtitle: String,
    pub pin_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitwardenImportReviewItem {
    pub source_index: usize,
    pub title: String,
    pub subtitle: String,
    pub similar_to: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BitwardenImportPreview {
    pub total_items: usize,
    pub importable_items: usize,
    pub similar_items: usize,
    pub skipped_items: usize,
    pub login_items: usize,
    pub secure_note_items: usize,
    pub card_items: usize,
    pub identity_items: usize,
    pub ssh_key_items: usize,
    pub similar_titles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitwardenImportError {
    Empty,
    InvalidJson,
    EncryptedExport,
    MissingItems,
}

impl std::fmt::Display for BitwardenImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Choose a Bitwarden JSON export first."),
            Self::InvalidJson => write!(f, "That does not look like Bitwarden JSON."),
            Self::EncryptedExport => {
                write!(f, "Encrypted Bitwarden exports are not supported yet.")
            }
            Self::MissingItems => write!(f, "No Bitwarden items were found."),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotesVault {
    pub notes: Vec<SecureNote>,
    pub next_id: u64,
}

impl Default for NotesVault {
    fn default() -> Self {
        Self {
            next_id: 1,
            notes: Vec::new(),
        }
    }
}

impl NotesVault {
    pub fn sample_data() -> Self {
        let mut vault = Self::default();
        vault.add_manual_item(
            "Sample Passport",
            "Identity",
            "",
            "Passport number: X1234567\nCountry: Exampleland\nExpires: 2032-04-15",
            "travel, id",
            false,
        );
        vault.add_manual_item(
            "Sample Insurance",
            "Secure Note",
            "",
            "Provider: Example Mutual\nPolicy: AUTO-000-123\nRoadside: 800-555-0100",
            "car, policy",
            false,
        );
        vault.add_manual_item(
            "Sample Device Serial",
            "Secure Note",
            "",
            "Device: Passport Prime\nSerial: PRIME-0000-EXAMPLE\nPurchased: 2026",
            "device",
            false,
        );
        for note in &mut vault.notes {
            note.updated_label = "Sample".to_string();
        }
        vault
    }

    pub fn add_manual_item(
        &mut self,
        title: impl Into<String>,
        item_type: impl Into<String>,
        account: impl Into<String>,
        secret: impl Into<String>,
        tags: impl Into<String>,
        pin_required: bool,
    ) -> u64 {
        self.add_manual_item_with_fields(title, item_type, account, secret, tags, "", pin_required)
    }

    pub fn add_manual_item_with_fields(
        &mut self,
        title: impl Into<String>,
        item_type: impl Into<String>,
        account: impl Into<String>,
        secret: impl Into<String>,
        tags: impl Into<String>,
        custom_fields: impl Into<String>,
        pin_required: bool,
    ) -> u64 {
        self.add_manual_item_full(
            title,
            item_type,
            account,
            secret,
            tags,
            custom_fields,
            "",
            false,
            pin_required,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_manual_item_full(
        &mut self,
        title: impl Into<String>,
        item_type: impl Into<String>,
        account: impl Into<String>,
        secret: impl Into<String>,
        tags: impl Into<String>,
        custom_fields: impl Into<String>,
        folder: impl Into<String>,
        favorite: bool,
        pin_required: bool,
    ) -> u64 {
        let id = self.next_id.max(1);
        self.next_id = id + 1;
        let item_type = parse_item_type(&item_type.into());
        let account = account.into().trim().to_string();
        let secret = secret.into().trim().to_string();
        let updated_rank = self.next_update_rank().max(id);
        let mut note = SecureNote {
            id,
            title: normalize_title(title.into()),
            category: item_type.label().to_string(),
            body: String::new(),
            tags: parse_tags(&tags.into()),
            folder: folder.into().trim().to_string(),
            favorite,
            updated_label: "Just now".to_string(),
            updated_rank,
            item_type,
            notes: String::new(),
            login: BitwardenLogin::default(),
            secure_note: BitwardenSecureNote::default(),
            card: BitwardenCard::default(),
            identity: BitwardenIdentity::default(),
            ssh_key: serde_json::Value::Null,
            fields: parse_custom_fields(&custom_fields.into()),
            archived: false,
            pin_required,
        };
        apply_manual_fields(&mut note, account, secret);
        self.notes.push(note);
        id
    }

    pub fn update_manual_item(
        &mut self,
        id: u64,
        title: impl Into<String>,
        item_type: impl Into<String>,
        account: impl Into<String>,
        secret: impl Into<String>,
        tags: impl Into<String>,
        pin_required: bool,
    ) -> bool {
        let custom_fields = self
            .note_by_id(id)
            .map(|note| fields_as_input(&note.fields))
            .unwrap_or_default();
        self.update_manual_item_with_fields(
            id,
            title,
            item_type,
            account,
            secret,
            tags,
            custom_fields,
            pin_required,
        )
    }

    pub fn update_manual_item_with_fields(
        &mut self,
        id: u64,
        title: impl Into<String>,
        item_type: impl Into<String>,
        account: impl Into<String>,
        secret: impl Into<String>,
        tags: impl Into<String>,
        custom_fields: impl Into<String>,
        pin_required: bool,
    ) -> bool {
        let (folder, favorite) = self
            .note_by_id(id)
            .map(|note| (note.folder.clone(), note.favorite))
            .unwrap_or_default();
        self.update_manual_item_full(
            id,
            title,
            item_type,
            account,
            secret,
            tags,
            custom_fields,
            folder,
            favorite,
            pin_required,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_manual_item_full(
        &mut self,
        id: u64,
        title: impl Into<String>,
        item_type: impl Into<String>,
        account: impl Into<String>,
        secret: impl Into<String>,
        tags: impl Into<String>,
        custom_fields: impl Into<String>,
        folder: impl Into<String>,
        favorite: bool,
        pin_required: bool,
    ) -> bool {
        let updated_rank = self.next_update_rank();
        let Some(note) = self.notes.iter_mut().find(|note| note.id == id) else {
            return false;
        };
        let item_type = parse_item_type(&item_type.into());
        let old_item_type = note.item_type;
        note.title = normalize_title(title.into());
        note.category = item_type.label().to_string();
        note.item_type = item_type;
        note.tags = parse_tags(&tags.into());
        note.folder = folder.into().trim().to_string();
        note.favorite = favorite;
        note.fields = parse_custom_fields(&custom_fields.into());
        note.pin_required = pin_required;
        note.updated_label = "Just now".to_string();
        note.updated_rank = updated_rank;
        let account = account.into().trim().to_string();
        let secret = secret.into().trim().to_string();
        if old_item_type == item_type {
            update_visible_manual_fields(note, account, secret);
        } else {
            apply_manual_fields(note, account, secret);
        }
        true
    }

    pub fn import_bitwarden_json(&mut self, input: &str) -> Result<usize, BitwardenImportError> {
        let selected = parse_bitwarden_export(input)?
            .items
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        self.import_bitwarden_json_selected(input, &selected)
    }

    pub fn import_bitwarden_json_selected(
        &mut self,
        input: &str,
        selected_source_indices: &[usize],
    ) -> Result<usize, BitwardenImportError> {
        let export = parse_bitwarden_export(input)?;
        let selected = selected_source_indices
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();

        let mut imported = 0;
        for (index, item) in export.items.iter().enumerate() {
            if !selected.contains(&index) {
                continue;
            }
            let id = self.next_id.max(1);
            let Some(mut note) = bitwarden_note_from_item(item, id, &export.folders) else {
                continue;
            };
            note.updated_rank = self.next_update_rank().max(id);
            self.next_id = id + 1;
            self.notes.push(note);
            imported += 1;
        }

        if imported == 0 {
            Err(BitwardenImportError::MissingItems)
        } else {
            Ok(imported)
        }
    }

    pub fn preview_bitwarden_json(
        &self,
        input: &str,
    ) -> Result<BitwardenImportPreview, BitwardenImportError> {
        let review = self.review_bitwarden_json(input)?;
        let mut preview = BitwardenImportPreview {
            total_items: parse_bitwarden_export(input)?.items.len(),
            ..BitwardenImportPreview::default()
        };

        for row in &review {
            let item_type = parse_item_type(&row.subtitle);
            match item_type {
                BitwardenItemType::Login => preview.login_items += 1,
                BitwardenItemType::SecureNote => preview.secure_note_items += 1,
                BitwardenItemType::Card => preview.card_items += 1,
                BitwardenItemType::Identity => preview.identity_items += 1,
                BitwardenItemType::SshKey => preview.ssh_key_items += 1,
            };
            preview.importable_items += 1;
            if !row.similar_to.is_empty() {
                preview.similar_items += 1;
                if preview.similar_titles.len() < 4 {
                    preview.similar_titles.push(row.title.clone());
                }
            }
        }

        preview.skipped_items = preview.total_items.saturating_sub(preview.importable_items);

        if preview.importable_items == 0 {
            Err(BitwardenImportError::MissingItems)
        } else {
            Ok(preview)
        }
    }

    pub fn review_bitwarden_json(
        &self,
        input: &str,
    ) -> Result<Vec<BitwardenImportReviewItem>, BitwardenImportError> {
        let export = parse_bitwarden_export(input)?;
        let mut rows = Vec::new();

        for (source_index, item) in export.items.iter().enumerate() {
            let Some(note) = bitwarden_note_from_item(item, source_index as u64 + 1, &export.folders)
            else {
                continue;
            };
            let similar_to = self.similar_existing_title(&note.title).unwrap_or_default();
            rows.push(BitwardenImportReviewItem {
                source_index,
                title: note.title,
                subtitle: note.item_type.label().to_string(),
                selected: similar_to.is_empty(),
                similar_to,
            });
        }

        rows.sort_by(|left, right| {
            let left_has_match = !left.similar_to.is_empty();
            let right_has_match = !right.similar_to.is_empty();
            right_has_match
                .cmp(&left_has_match)
                .then_with(|| left.title.to_ascii_lowercase().cmp(&right.title.to_ascii_lowercase()))
        });

        if rows.is_empty() {
            Err(BitwardenImportError::MissingItems)
        } else {
            Ok(rows)
        }
    }

    pub fn export_bitwarden_json(&self) -> serde_json::Value {
        let folders = self.export_folders();
        let folder_ids = folders
            .iter()
            .filter_map(|folder| Some((folder.name.clone()?, folder.id.clone()?)))
            .collect::<std::collections::BTreeMap<_, _>>();
        serde_json::json!({
            "encrypted": false,
            "folders": folders,
            "items": self.notes.iter().map(|note| {
                let folder_id = folder_ids.get(&note.folder).cloned();
                bitwarden_json_for_note(note, folder_id.as_deref())
            }).collect::<Vec<_>>(),
        })
    }

    pub fn archive_note(&mut self, id: u64) -> bool {
        let updated_rank = self.next_update_rank();
        let Some(note) = self.notes.iter_mut().find(|note| note.id == id) else {
            return false;
        };
        note.archived = true;
        note.updated_label = "Just now".to_string();
        note.updated_rank = updated_rank;
        true
    }

    pub fn restore_note(&mut self, id: u64) -> bool {
        let updated_rank = self.next_update_rank();
        let Some(note) = self.notes.iter_mut().find(|note| note.id == id) else {
            return false;
        };
        note.archived = false;
        note.updated_label = "Just now".to_string();
        note.updated_rank = updated_rank;
        true
    }

    pub fn delete_note(&mut self, id: u64) -> Option<SecureNote> {
        let index = self.notes.iter().position(|note| note.id == id)?;
        Some(self.notes.remove(index))
    }

    pub fn note_by_id(&self, id: u64) -> Option<&SecureNote> {
        self.notes.iter().find(|note| note.id == id)
    }

    pub fn active_ids(&self, query: &str) -> Vec<u64> {
        self.search_ids_with_filters(query, "", "", "", false)
    }

    pub fn archived_ids(&self, query: &str) -> Vec<u64> {
        self.search_ids_with_filters(query, "", "", "", true)
    }

    pub fn search_ids(&self, query: &str, archived: bool) -> Vec<u64> {
        self.search_ids_with_filters(query, "", "", "", archived)
    }

    pub fn active_ids_with_filters(
        &self,
        query: &str,
        folder: &str,
        item_type: &str,
        tag: &str,
    ) -> Vec<u64> {
        self.search_ids_with_filters(query, folder, item_type, tag, false)
    }

    pub fn search_ids_with_filters(
        &self,
        query: &str,
        folder: &str,
        item_type: &str,
        tag: &str,
        archived: bool,
    ) -> Vec<u64> {
        let query = query.trim().to_ascii_lowercase();
        let folder = normalize_filter_value(folder, "all folders");
        let item_type = normalize_filter_value(item_type, "all types");
        let tag = normalize_filter_value(tag, "all tags");
        self.notes
            .iter()
            .filter(|note| note.archived == archived)
            .filter(|note| query.is_empty() || note_matches(note, &query))
            .filter(|note| {
                folder.is_empty() || note.folder.trim().eq_ignore_ascii_case(folder.as_str())
            })
            .filter(|note| {
                item_type.is_empty() || note.item_type.label().eq_ignore_ascii_case(item_type.as_str())
            })
            .filter(|note| {
                tag.is_empty() || note.tags.iter().any(|note_tag| note_tag.eq_ignore_ascii_case(tag.as_str()))
            })
            .filter(|note| !note.title.trim().is_empty())
            .map(|note| note.id)
            .collect()
    }

    pub fn folder_options(&self) -> Vec<String> {
        unique_sorted_options(
            self.notes
                .iter()
                .filter(|note| !note.archived)
                .filter_map(|note| {
                    let folder = note.folder.trim();
                    if folder.is_empty() {
                        None
                    } else {
                        Some(folder.to_string())
                    }
                }),
        )
    }

    pub fn tag_options(&self) -> Vec<String> {
        unique_sorted_options(
            self.notes
                .iter()
                .filter(|note| !note.archived)
                .flat_map(|note| note.tags.iter().cloned()),
        )
    }

    pub fn summary(&self, id: u64) -> Option<NoteSummary> {
        let note = self.note_by_id(id)?;
        Some(NoteSummary {
            id: note.id,
            title: note.title.clone(),
            subtitle: note.item_type.label().to_string(),
            pin_required: note.pin_required,
        })
    }

    pub fn migrate_legacy_items(&mut self) -> bool {
        let mut changed = false;

        for note in &mut self.notes {
            if note.notes.is_empty() && !note.body.trim().is_empty() {
                note.notes = note.body.clone();
                changed = true;
            }

            let parsed_type = parse_item_type(&note.category);
            if note.item_type == BitwardenItemType::SecureNote
                && parsed_type != BitwardenItemType::SecureNote
            {
                note.item_type = parsed_type;
                changed = true;
            }

            let category = note.item_type.label().to_string();
            if note.category != category {
                note.category = category;
                changed = true;
            }

            let before_tags = note.tags.len();
            note.tags.retain(|tag| tag != "bitwarden");
            if note.tags.len() != before_tags {
                changed = true;
            }

            let body = render_note_body(note);
            if note.body != body {
                note.body = body;
                changed = true;
            }

            if note.updated_rank == 0 {
                note.updated_rank = note.id;
                changed = true;
            }
        }

        changed
    }

    pub fn deduplicate_by_title(&mut self) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        let before = self.notes.len();
        self.notes.retain(|note| {
            let title = normalize_for_match(&note.title);
            if title.is_empty() {
                return false;
            }
            seen.insert(title)
        });
        let removed = before.saturating_sub(self.notes.len());
        if let Some(max_id) = self.notes.iter().map(|note| note.id).max() {
            self.next_id = self.next_id.max(max_id + 1);
        } else {
            self.next_id = 1;
        }
        removed
    }

    fn similar_existing_title(&self, title: &str) -> Option<String> {
        self.notes
            .iter()
            .filter(|note| !note.archived)
            .filter_map(|note| {
                similarity_score(title, &note.title).map(|score| (score, note.title.clone()))
            })
            .max_by_key(|(score, _)| *score)
            .map(|(_, title)| title)
    }

    fn next_update_rank(&self) -> u64 {
        self.notes
            .iter()
            .map(|note| note.updated_rank.max(note.id))
            .max()
            .unwrap_or(0)
            + 1
    }

    fn export_folders(&self) -> Vec<BitwardenFolder> {
        self.notes
            .iter()
            .filter_map(|note| {
                let name = note.folder.trim();
                if name.is_empty() {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .fold(Vec::<String>::new(), |mut folders, folder| {
                if !folders.contains(&folder) {
                    folders.push(folder);
                }
                folders
            })
            .into_iter()
            .enumerate()
            .map(|(index, name)| BitwardenFolder {
                id: Some(stable_uuid(index as u64 + 1)),
                name: Some(name),
            })
            .collect()
    }
}

struct ParsedBitwardenExport {
    items: Vec<serde_json::Value>,
    folders: std::collections::HashMap<String, String>,
}

fn parse_bitwarden_export(input: &str) -> Result<ParsedBitwardenExport, BitwardenImportError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(BitwardenImportError::Empty);
    }

    let root: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|_| BitwardenImportError::InvalidJson)?;
    if root
        .get("encrypted")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return Err(BitwardenImportError::EncryptedExport);
    }

    let items = root
        .get("items")
        .and_then(|value| value.as_array())
        .ok_or(BitwardenImportError::MissingItems)?;

    let folders = root
        .get("folders")
        .and_then(|value| value.as_array())
        .map(|folders| {
            folders
                .iter()
                .filter_map(|folder| {
                    let folder: BitwardenFolder = serde_json::from_value(folder.clone()).ok()?;
                    Some((folder.id?, folder.name?))
                })
                .collect::<std::collections::HashMap<_, _>>()
        })
        .unwrap_or_default();

    Ok(ParsedBitwardenExport {
        items: items.clone(),
        folders,
    })
}

fn bitwarden_note_from_item(
    item: &serde_json::Value,
    id: u64,
    folders: &std::collections::HashMap<String, String>,
) -> Option<SecureNote> {
    let title = string_at(item, "name").unwrap_or_else(|| "Untitled Bitwarden item".to_string());
    let item_type = item
        .get("type")
        .and_then(|value| value.as_i64())
        .map(BitwardenItemType::from)
        .unwrap_or_default();

    let mut note = SecureNote {
        id,
        title: normalize_title(title),
            category: item_type.label().to_string(),
            body: String::new(),
            tags: vec!["imported".to_string()],
            folder: string_at(item, "folderId")
                .and_then(|folder_id| folders.get(&folder_id).cloned())
                .unwrap_or_default(),
            favorite: item
                .get("favorite")
                .and_then(|value| value.as_bool())
                .unwrap_or(false),
            updated_label: "Just now".to_string(),
            updated_rank: id,
            item_type,
            notes: string_at(item, "notes").unwrap_or_default(),
        login: item
            .get("login")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default(),
        secure_note: item
            .get("secureNote")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default(),
        card: item
            .get("card")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default(),
            identity: item
                .get("identity")
                .cloned()
                .map(serde_json::from_value)
                .transpose()
                .ok()
                .flatten()
                .unwrap_or_default(),
            ssh_key: item.get("sshKey").cloned().unwrap_or(serde_json::Value::Null),
            fields: item
                .get("fields")
                .cloned()
            .map(serde_json::from_value)
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default(),
            archived: false,
            pin_required: item
                .get("reprompt")
                .and_then(|value| value.as_i64())
                .unwrap_or(0)
                > 0,
        };
    note.body = render_note_body(&note);
    if note.body.trim().is_empty() {
        None
    } else {
        Some(note)
    }
}

pub fn detail_body(note: &SecureNote) -> String {
    let body = render_note_body(note);
    if body.trim().is_empty() {
        "No details yet.".to_string()
    } else {
        body
    }
}

pub fn information_body(note: &SecureNote) -> String {
    let body = render_note_body_without_fields(note);
    if body.trim().is_empty() {
        "No details yet.".to_string()
    } else {
        body
    }
}

pub fn tags_as_input(tags: &[String]) -> String {
    tags.join(", ")
}

pub fn fields_as_input(fields: &[BitwardenField]) -> String {
    fields
        .iter()
        .filter_map(|field| {
            let name = field.name.as_deref()?.trim();
            if name.is_empty() {
                return None;
            }
            let value = field.value.as_deref().unwrap_or("").trim();
            let prefix = match field.field_type.unwrap_or(0) {
                1 => "[Hidden] ",
                2 => "[Boolean] ",
                _ => "",
            };
            Some(if value.is_empty() {
                format!("{prefix}{name}:")
            } else {
                format!("{prefix}{name}: {value}")
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn note_type_input(note: &SecureNote) -> String {
    note.item_type.label().to_string()
}

pub fn account_input(note: &SecureNote) -> String {
    match note.item_type {
        BitwardenItemType::Login => note.login.username.clone().unwrap_or_default(),
        BitwardenItemType::Card => note.card.cardholder_name.clone().unwrap_or_default(),
        BitwardenItemType::Identity => note.identity.username.clone().unwrap_or_default(),
        BitwardenItemType::SecureNote => String::new(),
        BitwardenItemType::SshKey => string_at(&note.ssh_key, "publicKey").unwrap_or_default(),
    }
}

pub fn secret_input(note: &SecureNote) -> String {
    match note.item_type {
        BitwardenItemType::Login => note.login.password.clone().unwrap_or_default(),
        BitwardenItemType::Card => note.card.number.clone().unwrap_or_default(),
        BitwardenItemType::Identity => {
            if note.notes.is_empty() {
                note.identity
                    .ssn
                    .clone()
                    .or_else(|| note.identity.passport_number.clone())
                    .or_else(|| note.identity.license_number.clone())
                    .unwrap_or_default()
            } else {
                note.notes.clone()
            }
        }
        BitwardenItemType::SecureNote => {
            if note.notes.is_empty() {
                note.body.clone()
            } else {
                note.notes.clone()
            }
        }
        BitwardenItemType::SshKey => string_at(&note.ssh_key, "privateKey").unwrap_or_default(),
    }
}

fn apply_manual_fields(note: &mut SecureNote, account: String, secret: String) {
    note.login = BitwardenLogin::default();
    note.secure_note = BitwardenSecureNote::default();
    note.card = BitwardenCard::default();
    note.identity = BitwardenIdentity::default();
    note.ssh_key = serde_json::Value::Null;
    note.notes.clear();

    match note.item_type {
        BitwardenItemType::Login => {
            note.login.username = blank_to_none(account);
            note.login.password = blank_to_none(secret);
        }
        BitwardenItemType::Card => {
            note.card.cardholder_name = blank_to_none(account);
            note.card.number = blank_to_none(secret);
        }
        BitwardenItemType::Identity => {
            note.identity.username = blank_to_none(account);
            note.notes = secret;
        }
        BitwardenItemType::SecureNote => {
            note.notes = secret;
        }
        BitwardenItemType::SshKey => {
            note.ssh_key = serde_json::json!({
                "publicKey": blank_to_none(account),
                "privateKey": blank_to_none(secret),
                "keyFingerprint": serde_json::Value::Null,
            });
        }
    }

    note.body = render_note_body(note);
}

fn update_visible_manual_fields(note: &mut SecureNote, account: String, secret: String) {
    match note.item_type {
        BitwardenItemType::Login => {
            note.login.username = blank_to_none(account);
            note.login.password = blank_to_none(secret);
        }
        BitwardenItemType::Card => {
            note.card.cardholder_name = blank_to_none(account);
            note.card.number = blank_to_none(secret);
        }
        BitwardenItemType::Identity => {
            note.identity.username = blank_to_none(account);
            note.notes = secret;
        }
        BitwardenItemType::SecureNote => {
            note.notes = secret;
        }
        BitwardenItemType::SshKey => {
            let mut ssh_key = note.ssh_key.clone();
            if !ssh_key.is_object() {
                ssh_key = serde_json::json!({});
            }
            ssh_key["publicKey"] = blank_to_none(account)
                .map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null);
            ssh_key["privateKey"] = blank_to_none(secret)
                .map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null);
            if ssh_key.get("keyFingerprint").is_none() {
                ssh_key["keyFingerprint"] = serde_json::Value::Null;
            }
            note.ssh_key = ssh_key;
        }
    }

    note.body = render_note_body(note);
}

fn bitwarden_json_for_note(note: &SecureNote, folder_id: Option<&str>) -> serde_json::Value {
    let mut item = serde_json::json!({
        "type": i64::from(note.item_type),
        "name": note.title,
        "notes": note.notes,
        "fields": note.fields,
        "favorite": note.favorite,
        "folderId": folder_id,
        "reprompt": if note.pin_required { 1 } else { 0 },
    });

    match note.item_type {
        BitwardenItemType::Login => item["login"] = serde_json::json!(note.login),
        BitwardenItemType::SecureNote => item["secureNote"] = serde_json::json!(note.secure_note),
        BitwardenItemType::Card => item["card"] = serde_json::json!(note.card),
        BitwardenItemType::Identity => item["identity"] = serde_json::json!(note.identity),
        BitwardenItemType::SshKey => item["sshKey"] = note.ssh_key.clone(),
    }
    item
}

fn render_note_body(note: &SecureNote) -> String {
    render_note_body_inner(note, true)
}

fn render_note_body_without_fields(note: &SecureNote) -> String {
    render_note_body_inner(note, false)
}

fn render_note_body_inner(note: &SecureNote, include_custom_fields: bool) -> String {
    let mut lines = Vec::new();
    match note.item_type {
        BitwardenItemType::Login => {
            push_line(&mut lines, "Username", note.login.username.as_deref());
            push_line(&mut lines, "Password", note.login.password.as_deref());
            push_line(&mut lines, "TOTP", note.login.totp.as_deref());
            for (index, uri) in note.login.uris.iter().enumerate() {
                push_line(
                    &mut lines,
                    &format!("URL {}", index + 1),
                    uri.uri.as_deref(),
                );
            }
        }
        BitwardenItemType::SecureNote => {}
        BitwardenItemType::Card => {
            push_line(
                &mut lines,
                "Cardholder",
                note.card.cardholder_name.as_deref(),
            );
            push_line(&mut lines, "Brand", note.card.brand.as_deref());
            push_line(&mut lines, "Number", note.card.number.as_deref());
            push_line(
                &mut lines,
                "Expiration month",
                note.card.exp_month.as_deref(),
            );
            push_line(&mut lines, "Expiration year", note.card.exp_year.as_deref());
            push_line(&mut lines, "Security code", note.card.code.as_deref());
        }
        BitwardenItemType::Identity => {
            push_line(&mut lines, "Title", note.identity.title.as_deref());
            push_line(
                &mut lines,
                "First name",
                note.identity.first_name.as_deref(),
            );
            push_line(
                &mut lines,
                "Middle name",
                note.identity.middle_name.as_deref(),
            );
            push_line(&mut lines, "Last name", note.identity.last_name.as_deref());
            push_line(&mut lines, "Username", note.identity.username.as_deref());
            push_line(&mut lines, "Company", note.identity.company.as_deref());
            push_line(&mut lines, "SSN", note.identity.ssn.as_deref());
            push_line(
                &mut lines,
                "Passport number",
                note.identity.passport_number.as_deref(),
            );
            push_line(
                &mut lines,
                "License number",
                note.identity.license_number.as_deref(),
            );
            push_line(&mut lines, "Email", note.identity.email.as_deref());
            push_line(&mut lines, "Phone", note.identity.phone.as_deref());
            push_line(&mut lines, "Address 1", note.identity.address1.as_deref());
            push_line(&mut lines, "Address 2", note.identity.address2.as_deref());
            push_line(&mut lines, "Address 3", note.identity.address3.as_deref());
            push_line(&mut lines, "City", note.identity.city.as_deref());
            push_line(&mut lines, "State", note.identity.state.as_deref());
            push_line(
                &mut lines,
                "Postal code",
                note.identity.postal_code.as_deref(),
            );
            push_line(&mut lines, "Country", note.identity.country.as_deref());
        }
        BitwardenItemType::SshKey => {
            if let Some(public_key) = string_at(&note.ssh_key, "publicKey") {
                push_line(&mut lines, "Public key", Some(&public_key));
            }
            if let Some(private_key) = string_at(&note.ssh_key, "privateKey") {
                push_line(&mut lines, "Private key", Some(&private_key));
            }
            if let Some(fingerprint) = string_at(&note.ssh_key, "keyFingerprint")
                .or_else(|| string_at(&note.ssh_key, "fingerprint"))
            {
                push_line(&mut lines, "Fingerprint", Some(&fingerprint));
            }
        }
    }

    if !note.notes.trim().is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.extend(note.notes.lines().map(|line| line.trim_end().to_string()));
    }

    if !note.body.trim().is_empty() && note.notes.trim().is_empty() && lines.is_empty() {
        lines.extend(note.body.lines().map(|line| line.trim_end().to_string()));
    }

    if include_custom_fields {
        for field in &note.fields {
            if let Some(name) = field.name.as_deref() {
                push_line(&mut lines, name, field.value.as_deref());
            }
        }
    }

    lines
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn note_matches(note: &SecureNote, query: &str) -> bool {
    note.title.to_ascii_lowercase().contains(query)
        || note.item_type.label().to_ascii_lowercase().contains(query)
        || note.category.to_ascii_lowercase().contains(query)
        || note.folder.to_ascii_lowercase().contains(query)
        || note.body.to_ascii_lowercase().contains(query)
        || note.notes.to_ascii_lowercase().contains(query)
        || render_note_body(note).to_ascii_lowercase().contains(query)
        || note
            .tags
            .iter()
            .any(|tag| tag.to_ascii_lowercase().contains(query))
}

fn normalize_title(title: impl Into<String>) -> String {
    let title = title.into();
    let trimmed = title.trim();
    if trimmed.is_empty() {
        "Untitled item".to_string()
    } else {
        trimmed.to_string()
    }
}

fn normalize_for_match(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn similarity_score(left: &str, right: &str) -> Option<u8> {
    let left = normalize_for_match(left);
    let right = normalize_for_match(right);
    if left.is_empty() || right.is_empty() {
        return None;
    }
    if left == right {
        return Some(100);
    }
    if left.contains(&right) || right.contains(&left) {
        return Some(85);
    }

    let left_tokens = left
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|token| token.len() > 2)
        .collect::<std::collections::BTreeSet<_>>();
    let right_tokens = right
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|token| token.len() > 2)
        .collect::<std::collections::BTreeSet<_>>();
    if left_tokens.is_empty() || right_tokens.is_empty() {
        return None;
    }

    let shared = left_tokens.intersection(&right_tokens).count();
    let smaller = left_tokens.len().min(right_tokens.len());
    if shared * 3 >= smaller * 2 {
        Some(70)
    } else {
        None
    }
}

fn parse_item_type(input: &str) -> BitwardenItemType {
    match input.trim().to_ascii_lowercase().as_str() {
        "login" | "password" => BitwardenItemType::Login,
        "card" | "payment card" | "credit card" => BitwardenItemType::Card,
        "identity" => BitwardenItemType::Identity,
        "ssh key" | "ssh" => BitwardenItemType::SshKey,
        _ => BitwardenItemType::SecureNote,
    }
}

fn parse_tags(input: &str) -> Vec<String> {
    input
        .split([',', '#'])
        .map(|part| {
            part.trim()
                .trim_start_matches('#')
                .to_ascii_lowercase()
                .replace(' ', "-")
        })
        .filter(|part| !part.is_empty())
        .fold(Vec::new(), |mut tags, tag| {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
            tags
        })
}

fn stable_uuid(id: u64) -> String {
    format!("00000000-0000-4000-8000-{id:012x}")
}

fn normalize_filter_value(input: &str, all_label: &str) -> String {
    let value = input.trim();
    if value.is_empty() || value.eq_ignore_ascii_case(all_label) {
        String::new()
    } else {
        value.to_string()
    }
}

fn unique_sorted_options(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut values = values
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .fold(Vec::<String>::new(), |mut options, option| {
            if !options
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(option.as_str()))
            {
                options.push(option);
            }
            options
        });
    values.sort_by_key(|value| value.to_ascii_lowercase());
    values
}

fn parse_custom_fields(input: &str) -> Vec<BitwardenField> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (field_type, line) = if let Some(rest) = line.strip_prefix("[Hidden]") {
                (Some(1), rest.trim())
            } else if let Some(rest) = line.strip_prefix("[Boolean]") {
                (Some(2), rest.trim())
            } else {
                (Some(0), line)
            };
            let (name, value) = line.split_once(':')?;
            let name = name.trim();
            if name.is_empty() {
                return None;
            }
            Some(BitwardenField {
                name: Some(name.to_string()),
                value: blank_to_none(value.to_string()),
                field_type,
            })
        })
        .collect()
}

fn string_at(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn push_line(lines: &mut Vec<String>, label: &str, value: Option<&str>) {
    if let Some(value) = value {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            lines.push(format!("{label}: {trimmed}"));
        }
    }
}

fn blank_to_none(value: String) -> Option<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_vault() -> NotesVault {
        NotesVault::sample_data()
    }

    #[test]
    fn default_vault_starts_empty_for_real_user_data() {
        let vault = NotesVault::default();

        assert!(vault.notes.is_empty());
        assert_eq!(vault.next_id, 1);
    }

    #[test]
    fn searches_active_items_without_archived_items() {
        let mut vault = sample_vault();
        assert_eq!(vault.active_ids("passport"), vec![1, 3]);

        vault.archive_note(1);

        assert_eq!(vault.active_ids("passport"), vec![3]);
        assert_eq!(vault.archived_ids("passport"), vec![1]);
    }

    #[test]
    fn add_item_normalizes_empty_fields_and_tags() {
        let mut vault = NotesVault {
            notes: Vec::new(),
            next_id: 1,
        };
        let id = vault.add_manual_item(
            " ",
            "Login",
            "alex",
            "correct horse",
            "bank, Bank, #tax docs",
            true,
        );
        let note = vault.note_by_id(id).unwrap();

        assert_eq!(note.title, "Untitled item");
        assert_eq!(note.item_type, BitwardenItemType::Login);
        assert_eq!(note.login.username.as_deref(), Some("alex"));
        assert!(note.pin_required);
        assert_eq!(note.tags, vec!["bank", "tax-docs"]);
    }

    #[test]
    fn manual_items_store_bitwarden_compatible_custom_fields() {
        let mut vault = NotesVault {
            notes: Vec::new(),
            next_id: 1,
        };
        let id = vault.add_manual_item_with_fields(
            "Insurance",
            "Secure Note",
            "",
            "Provider: Example Mutual",
            "car",
            "Policy number: AUTO-123\nSupport phone: 800-555-0100\nignored line",
            false,
        );

        let note = vault.note_by_id(id).unwrap();
        assert_eq!(note.fields.len(), 2);
        assert_eq!(note.fields[0].name.as_deref(), Some("Policy number"));
        assert_eq!(note.fields[0].value.as_deref(), Some("AUTO-123"));
        assert_eq!(
            fields_as_input(&note.fields),
            "Policy number: AUTO-123\nSupport phone: 800-555-0100"
        );
        assert!(detail_body(note).contains("Policy number: AUTO-123"));
        assert!(!information_body(note).contains("Policy number: AUTO-123"));

        assert!(vault.update_manual_item_with_fields(
            id,
            "Insurance",
            "Secure Note",
            "",
            "Provider: Example Mutual",
            "car",
            "Policy number: AUTO-456",
            false
        ));
        assert_eq!(
            vault.note_by_id(id).unwrap().fields[0].value.as_deref(),
            Some("AUTO-456")
        );
    }

    #[test]
    fn update_archive_restore_and_delete_item_by_id() {
        let mut vault = sample_vault();

        assert!(
            vault.update_manual_item(1, "Passport", "Identity", "alex", "9999", "travel", false)
        );
        assert_eq!(vault.note_by_id(1).unwrap().title, "Passport");
        assert!(vault.archive_note(1));
        assert!(vault.note_by_id(1).unwrap().archived);
        assert!(vault.restore_note(1));
        assert!(!vault.note_by_id(1).unwrap().archived);
        assert!(vault.delete_note(1).is_some());
        assert!(vault.note_by_id(1).is_none());
    }

    #[test]
    fn migrates_legacy_category_and_body_fields() {
        let mut vault = NotesVault {
            notes: vec![SecureNote {
                id: 1,
                title: "Legacy Passport".to_string(),
                category: "Identity".to_string(),
                body: "Passport number: X1234567".to_string(),
                tags: Vec::new(),
                folder: String::new(),
                favorite: false,
                updated_label: "Sample".to_string(),
                updated_rank: 1,
                item_type: BitwardenItemType::SecureNote,
                notes: String::new(),
                login: BitwardenLogin::default(),
                secure_note: BitwardenSecureNote::default(),
                card: BitwardenCard::default(),
                identity: BitwardenIdentity::default(),
                ssh_key: serde_json::Value::Null,
                fields: Vec::new(),
                archived: false,
                pin_required: false,
            }],
            next_id: 2,
        };

        assert!(vault.migrate_legacy_items());
        assert_eq!(vault.notes[0].item_type, BitwardenItemType::Identity);
        assert_eq!(vault.notes[0].category, "Identity");
        assert_eq!(vault.notes[0].notes, "Passport number: X1234567");
    }

    #[test]
    fn imports_bitwarden_json_items_without_flattening_types() {
        let json = r#"{
            "encrypted": false,
            "items": [
                {
                    "type": 1,
                    "name": "Example Login",
                    "notes": "Recovery code: ABCD",
                    "login": {
                    "username": "alex",
                        "password": "correct horse",
                        "uris": [{ "uri": "https://example.com" }]
                    },
                    "fields": [{ "name": "Account number", "value": "1234", "type": 0 }]
                },
                {
                    "type": 3,
                    "name": "Example Card",
                    "card": {
                        "cardholderName": "Example Person",
                        "brand": "Visa",
                        "number": "4111111111111111",
                        "expMonth": "01",
                        "expYear": "2030",
                        "code": "123"
                    }
                }
            ]
        }"#;
        let mut vault = NotesVault {
            notes: Vec::new(),
            next_id: 1,
        };

        assert_eq!(vault.import_bitwarden_json(json).unwrap(), 2);
        assert_eq!(vault.notes[0].title, "Example Login");
        assert_eq!(vault.notes[0].item_type, BitwardenItemType::Login);
        assert_eq!(vault.notes[0].login.username.as_deref(), Some("alex"));
        assert_eq!(
            vault.notes[0].fields[0].name.as_deref(),
            Some("Account number")
        );
        assert!(detail_body(&vault.notes[0]).contains("Recovery code: ABCD"));
        assert_eq!(vault.notes[1].item_type, BitwardenItemType::Card);
    }

    #[test]
    fn previews_bitwarden_json_before_importing() {
        let json = r#"{
            "encrypted": false,
            "items": [
                {
                    "type": 1,
                    "name": "Sample Passport",
                "login": { "username": "alex", "password": "correct horse" }
                },
                {
                    "type": 4,
                    "name": "Passport Identity",
                    "identity": { "passportNumber": "X1234567" }
                }
            ]
        }"#;
        let vault = sample_vault();

        let preview = vault.preview_bitwarden_json(json).unwrap();

        assert_eq!(preview.total_items, 2);
        assert_eq!(preview.importable_items, 2);
        assert_eq!(preview.login_items, 1);
        assert_eq!(preview.identity_items, 1);
        assert_eq!(preview.similar_items, 1);
        assert_eq!(preview.similar_titles, vec!["Sample Passport"]);
    }

    #[test]
    fn editing_imported_items_preserves_hidden_structured_fields() {
        let json = r#"{
            "encrypted": false,
            "items": [
                {
                    "type": 1,
                    "name": "Developer Login",
                    "login": {
                        "username": "old-user",
                        "password": "old-pass",
                        "totp": "otpauth://totp/example",
                        "uris": [{ "uri": "https://example.com" }]
                    }
                },
                {
                    "type": 3,
                    "name": "Travel Card",
                    "card": {
                        "cardholderName": "Old Name",
                        "brand": "Visa",
                        "number": "4111111111111111",
                        "expMonth": "01",
                        "expYear": "2030",
                        "code": "123"
                    }
                }
            ]
        }"#;
        let mut vault = NotesVault::default();
        vault.import_bitwarden_json(json).unwrap();

        assert!(vault.update_manual_item(
            1,
            "Developer Login",
            "Login",
            "new-user",
            "new-pass",
            "work",
            true
        ));
        assert_eq!(vault.notes[0].login.username.as_deref(), Some("new-user"));
        assert_eq!(vault.notes[0].login.password.as_deref(), Some("new-pass"));
        assert_eq!(
            vault.notes[0].login.totp.as_deref(),
            Some("otpauth://totp/example")
        );
        assert_eq!(
            vault.notes[0].login.uris[0].uri.as_deref(),
            Some("https://example.com")
        );

        assert!(vault.update_manual_item(
            2,
            "Travel Card",
            "Payment Card",
            "New Name",
            "5555555555554444",
            "travel",
            false
        ));
        assert_eq!(vault.notes[1].card.cardholder_name.as_deref(), Some("New Name"));
        assert_eq!(vault.notes[1].card.number.as_deref(), Some("5555555555554444"));
        assert_eq!(vault.notes[1].card.brand.as_deref(), Some("Visa"));
        assert_eq!(vault.notes[1].card.exp_month.as_deref(), Some("01"));
        assert_eq!(vault.notes[1].card.exp_year.as_deref(), Some("2030"));
        assert_eq!(vault.notes[1].card.code.as_deref(), Some("123"));
    }

    #[test]
    fn exports_bitwarden_shaped_json() {
        let mut vault = NotesVault {
            notes: Vec::new(),
            next_id: 1,
        };
        vault.add_manual_item("Example Login", "Login", "alex", "correct horse", "", false);
        let export = vault.export_bitwarden_json();

        assert_eq!(export["encrypted"], false);
        assert_eq!(export["items"][0]["type"], 1);
        assert_eq!(export["items"][0]["name"], "Example Login");
        assert_eq!(export["items"][0]["login"]["username"], "alex");
    }

    #[test]
    fn rejects_encrypted_bitwarden_export() {
        let mut vault = NotesVault {
            notes: Vec::new(),
            next_id: 1,
        };
        let result = vault.import_bitwarden_json(r#"{"encrypted":true,"items":[]}"#);

        assert_eq!(result, Err(BitwardenImportError::EncryptedExport));
    }
}
