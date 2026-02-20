# THE DARK FOREST - Interactive Adventure
# Fully playable CYOA game

out("╔════════════════════════════════════════╗")
out("║      THE DARK FOREST                   ║")
out("║  A Choose Your Own Adventure           ║")
out("╚════════════════════════════════════════╝")
out("")

# Player state
int | health -> 100
int | gold -> 0
str | weapon -> "fists"

# Show stats
out("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
out(concat("Health: ", health, " | Gold: ", gold, " | Weapon: ", weapon))
out("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
out("")

# Start
out("You wake up in a dark forest.")
out("A dirt path splits in two directions.")
out("")
out("What do you do?")
out("  1. Take the left path")
out("  2. Take the right path")
out("  3. Climb a tree")
out("")

str | choice1 -> cli_prompt("Your choice (1-3): ")
out("")

cond(choice1) {
    |> choice1 == "1" : {
        out("You take the LEFT path...")
        out("")
        out("You find a glowing sword stuck in a stone!")
        out("")
        out("What do you do?")
        out("  1. Pull out the sword")
        out("  2. Leave it")
        out("")
        
        str | choice2 -> cli_prompt("Your choice (1-2): ")
        out("")
        
        cond(choice2) {
            |> choice2 == "1" : {
                out("You pull out the sword!")
                out("✨ You obtained: GLOWING SWORD!")
                weapon = "Glowing Sword"
            }
            |> else : {
                out("You leave the sword behind.")
            }
        }
    }
    |> choice1 == "2" : {
        out("You take the RIGHT path...")
        out("")
        out("You meet a friendly merchant!")
        out("")
        out("What do you do?")
        out("  1. Trade with him")
        out("  2. Attack him")
        out("")
        
        str | choice2 -> cli_prompt("Your choice (1-2): ")
        out("")
        
        cond(choice2) {
            |> choice2 == "1" : {
                out("The merchant gives you 50 gold!")
                out("✨ You obtained: 50 GOLD!")
                gold = gold + 50
            }
            |> else : {
                out("You attack the merchant!")
                out("He fights back!")
                out("💔 You lost 30 health!")
                health = health - 30
            }
        }
    }
    |> else : {
        out("You CLIMB the tree...")
        out("")
        out("From here you see a village!")
        out("You climb down and head there.")
    }
}

out("")
out("You continue through the forest...")
out("")
out("A WILD BEAST appears!")
out("")

cond(weapon) {
    |> weapon == "Glowing Sword" : {
        out("You draw your sword!")
        out("The beast runs away in fear!")
        out("✨ You found 30 gold!")
        gold = gold + 30
    }
    |> else : {
        out("You have no weapon!")
        out("The beast attacks!")
        out("💔 You lost 50 health!")
        health = health - 50
    }
}

out("")

cond(health) {
    |> health <= 0 : {
        out("╔════════════════════════════════════════╗")
        out("║         GAME OVER                      ║")
        out("╚════════════════════════════════════════╝")
        out("")
        out("You died in the forest...")
        out(concat("Final Gold: ", gold))
    }
    |> else : {
        out("You reach the village safely!")
        out("")
        out("╔════════════════════════════════════════╗")
        out("║      CONGRATULATIONS!                  ║")
        out("╚════════════════════════════════════════╝")
        out("")
        out("═══ FINAL STATS ═══")
        out(concat("Health: ", health))
        out(concat("Gold: ", gold))
        out(concat("Weapon: ", weapon))
        out("═══════════════════")
        out("")
        
        str | save_data -> concat("health=", health, ",gold=", gold)
        file_write("save.txt", save_data)
        out("Game saved!")
    }
}

out("")
out("Thanks for playing!")
