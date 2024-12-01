
Planetary Flood/Drought/Earthquake/Famine: "A major flood on [PLANET_NAME] in your [NEBULA] Nebula has caused major damages to the local populace. Your people call for your aid."
- OPTIONA - Provide Aid: "Send resources and help to [PLANET_NAME] at the cost of [X ENERGY]" -> Lose [X ENERGY]
- OPTIONB - This too shall pass: "There are other more pressing matters. The people will prevail" -> Lose [X POPULATION], [-NEBULA HAPPYNESS]
- OPTIONC - Accept Aid: "[FACTION] offers to aid this [NEBULA] to reflect our friendship" -> Lose [FACTION STANDING]
 - CONDITION - [FACTION] must have standing >= 4
- OPTIOND - Kick them while they're down: "[FACTION] has wronged us before. Let us assymilate this system while it is weak" -> Start battle at [NEBULA], [BATTLE BONUS], lose [X ENERGY]

A Neighbour in need...: "The people of the [NEBULA] Nebula, held by [FACTION] have been struck by disaster, and are asking for aid"
- OPTIONA - Assist: "Send some resources to assist, at the cost of [X ENERGY]" -> Lose [X ENERGY], Gain [FACTION STANDING]
- OPTIONB - Undermine: "Send large amounts of aid, alongside some propaganda..." -> Lose [X ENERGY (more)], [-NEBULA HAPPYNESS], Gain [FACTION STANDING]
- OPTIONC - Ignore: "It is not our responsibility after all..."

Pirates!: "A battalion of rogues threaten your Nebula [NEBULA]. They demand a steep sum..."
 - OPTIONA - Relent: "Give them what they want. The peoples safety comes first" -> Lose [X ENERGY], [+NEBULA HAPPYNESS]
 - OPTIONB - Reinforce: "The pirates can try their luck against our guns." -> Lose [X ENERGY (less)], Lose [X POPULATION]
 - OPTIONC - Call the bluff: "No-one is going to throw us around. We know better than to play their games." -> Lose [X POPULATION], [-NEBULA HAPPYNESS]

In the shadows: "An opportunity opens up to send your spies to [FACTION] to get the upper hand in battle."
    - CONDITION - Must be at battle with [FACTION]
 - OPTIONA - Capital Directive: "Send in your best spies to gain intel without being detected" -> Lose [X ENERGY], Gain [BATTLE BONUS]
 - OPTIONB - Minor Directive: "Send in troops to quietly gain intel, but they might get caught" -> Lose [X ENERGY (less)], 50/50 gain [BATTLE BONUS]/lose [X POPULATION][FACTION STANDING]
 - OPTIONC - Waste of Time: "Let our frontline win the battle outright, we don't need intel." -> Nothing

Imposter: "You've been made aware of a mole within your ranks: a spy from [FACTION]."
 - OPTIONA - Ransom: "Demand that [FACTION] pay for the safe return of their agent - if they even care..." -> 50/50 gain [X ENERGY]/Nothing
 - OPTIONB - Make 'Em Talk: "Make this agent be of use to us before they're sent out of the airlock." -> gain [BATTLE BONUS], lose [FACTION STANDING]
 - OPTIONC - A Fair Exchange: "We give them their agent, they give us back our prisoners, and no one gets hurt." -> gain [FACTION STANDING]

Rebel Coup: "A rebellion is stirring in the [NEBULA] nebula - it cannot be left unchecked"
 - OPTIONA - Set an example: "Tighten our grip on [NEBULA], Cut off their connection to the Daedalus network. Let's show the people how to behave..." -> Lose [X POPULATION], LOSE [X ENERGY PROD], N of your NEBULAS GAIN 1 HAPPYNESS
 - OPTIONB - Send in the troops: "We will meet the rebels with the force they deserve, even it it affects production" -> Start battle at [NEBULA] with a WC of 80%
 - OPTIONC - Negotiate with the Rebels: "We can sympathise with the rebels, let us come to an agreement -> LOSE [X/Y/Z ENERGY], [++NEBULA HAPPYNESS]

