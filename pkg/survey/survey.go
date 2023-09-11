package survey

import (
	"fmt"

	"github.com/AlecAivazis/survey/v2"
)

func Execute() {
	days := []string{}
	prompt := &survey.MultiSelect{
		Message: "What days do you prefer:",
		Options: []string{"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"},
	}
	survey.AskOne(prompt, &days)

	fmt.Println("Hello", days)
}
