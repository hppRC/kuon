// type Tweet struct {
// 	Contributors                []int64                `json:"contributors"`
// 	Coordinates                 *Coordinates           `json:"coordinates"`
// 	CreatedAt                   string                 `json:"created_at"`
// 	DisplayTextRange            []int                  `json:"display_text_range"`
// 	Entities                    Entities               `json:"entities"`
// 	ExtendedEntities            Entities               `json:"extended_entities"`
// 	ExtendedTweet               ExtendedTweet          `json:"extended_tweet"`
// 	FavoriteCount               int                    `json:"favorite_count"`
// 	Favorited                   bool                   `json:"favorited"`
// 	FilterLevel                 string                 `json:"filter_level"`
// 	FullText                    string                 `json:"full_text"`
// 	HasExtendedProfile          bool                   `json:"has_extended_profile"`
// 	Id                          int64                  `json:"id"`
// 	IdStr                       string                 `json:"id_str"`
// 	InReplyToScreenName         string                 `json:"in_reply_to_screen_name"`
// 	InReplyToStatusID           int64                  `json:"in_reply_to_status_id"`
// 	InReplyToStatusIdStr        string                 `json:"in_reply_to_status_id_str"`
// 	InReplyToUserID             int64                  `json:"in_reply_to_user_id"`
// 	InReplyToUserIdStr          string                 `json:"in_reply_to_user_id_str"`
// 	IsTranslationEnabled        bool                   `json:"is_translation_enabled"`
// 	Lang                        string                 `json:"lang"`
// 	Place                       Place                  `json:"place"`
// 	QuotedStatusID              int64                  `json:"quoted_status_id"`
// 	QuotedStatusIdStr           string                 `json:"quoted_status_id_str"`
// 	QuotedStatus                *Tweet                 `json:"quoted_status"`
// 	PossiblySensitive           bool                   `json:"possibly_sensitive"`
// 	PossiblySensitiveAppealable bool                   `json:"possibly_sensitive_appealable"`
// 	RetweetCount                int                    `json:"retweet_count"`
// 	Retweeted                   bool                   `json:"retweeted"`
// 	RetweetedStatus             *Tweet                 `json:"retweeted_status"`
// 	Source                      string                 `json:"source"`
// 	Scopes                      map[string]interface{} `json:"scopes"`
// 	Text                        string                 `json:"text"`
// 	User                        User                   `json:"user"`
// 	WithheldCopyright           bool                   `json:"withheld_copyright"`
// 	WithheldInCountries         []string               `json:"withheld_in_countries"`
// 	WithheldScope               string                 `json:"withheld_scope"`
// 	//Geo is deprecated
// 	//Geo                  interface{} `json:"geo"`
// }
pub struct Tweet {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
