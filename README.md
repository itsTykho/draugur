
# Draugur - EVE Killmail Discord Bot

A dead simple Discord bot that tracks killmails in real-time and posts them to Discord channels, not trying to be anything fancier.

Built with Rust using the Serenity framework and zKill's RedisQ service.

[Invite Link](https://discord.com/oauth2/authorize?client_id=1200488782110138488&permissions=84032&integration_type=0&scope=applications.commands+bot)

Built by [Tykho](https://zkillboard.com/character/2113678525/), ISK donations are always appreciated!

___

## Why another Discord bot?

- A return to better looking embeds
- Fun activity status - tracks most expensive kill across New Eden for the last 10 minutes

## Commands

### `/setup <follow_id>`
Add an entity ID to your server's tracking list. Works for characters, corps, systems, alliances, or a ship type.

### `/remove <follow_id>`
Remove an entity ID from your server's tracking list.

### `/list`
Display all currently tracked entity IDs and the channel they post to.

## Finding Entity IDs

Go to `zkillboard.com`, search for whatever you'd like to track. In the URL, there will be a bunch of numbers at the end - that is the ID you're looking for.

## Acknowledgments

- [zKillboard](https://zkillboard.com/) for providing the RedisQ killmail feed
- [CCP Games](https://www.ccpgames.com/) for EVE Online and the ESI API

## Support

- Create an issue on GitHub for bugs or feature requests

---

**Note**: This bot is not affiliated with CCP Games or EVE Online. EVE Online is a trademark of CCP hf.
