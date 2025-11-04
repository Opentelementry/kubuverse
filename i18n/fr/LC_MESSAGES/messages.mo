package LC_MESSAGES

import "github.com/nicksnyder/go-i18n/v2/i18n"

// French translations
var Messages = []*i18n.Message{
	{
		ID:    "HelloWorld",
		Other: "Bonjour le monde",
	},
	{
		ID:    "WelcomeUser",
		Other: "Bienvenue, {{.Name}}!",
	},
	{
		ID:    "ErrorNotFound",
		Other: "Erreur : élément introuvable.",
	},
	{
		ID:    "SubmitSuccess",
		Other: "Votre soumission a été envoyée avec succès.",
	},
}
