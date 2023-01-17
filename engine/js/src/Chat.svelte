<script>
    import {adminRequest} from './util.js';
    import Nav from './Nav.svelte';
    import {onMount} from 'svelte';

    let alias = "Pancake";
    let message = "";

    let players = [];
    onMount(async () => {
        const response = await adminRequest('RequestPlayers');
        if (response.PlayersRequested) {
            players = response.PlayersRequested;
        }
    });

    async function overrideAlias(playerId, previousAlias) {
        const alias = prompt("Override alias to? (ok to confirm)", previousAlias);
        const response = await adminRequest({OverridePlayerAlias: {player_id: playerId, alias}});
        if (response.PlayerAliasOverridden) {
            const player = players.find(p => p.player_id == playerId);
            if (player != null) {
                player.alias = response.PlayerAliasOverridden;

                // Reactivity
                players = players;
            }
        }
    }

    async function overrideModerator(playerId, moderator) {
        const response = await adminRequest({OverridePlayerModerator: {player_id: playerId, moderator}});
        if (typeof response.PlayerModeratorOverridden === 'boolean') {
            const player = players.find(p => p.player_id == playerId);
            if (player != null) {
                player.moderator = response.PlayerModeratorOverridden;

                // Reactivity
                players = players;
            }
        }
    }

    async function mute(playerId, minutes) {
        const response = await adminRequest({MutePlayer: {player_id: playerId, minutes}});
        if (typeof response.PlayerMuted === 'number') {
            const player = players.find(p => p.player_id == playerId);
            if (player != null) {
                player.mute = response.PlayerMuted;

                // Reactivity
                players = players;
            }
        }
    }

    async function restrict(playerId, minutes) {
        const response = await adminRequest({RestrictPlayer: {player_id: playerId, minutes}});
        if (typeof response.PlayerRestricted === 'number') {
            const player = players.find(p => p.player_id == playerId);
            if (player != null) {
                player.restriction = response.PlayerRestricted;

                // Reactivity
                players = players;
            }
        }
    }

    async function sendChat(player_id) {
        if (!alias || alias.length == 0 || !message || message.length == 0) {
            return;
        }
        const response = await adminRequest({SendChat: {alias, message, player_id}});
        if (response == "ChatSent") {
            message = "";
        }
    }

    const PRESETS = {
        "Custom Message": ""
    }

    /// Replaces null with question mark.
    function maybe(val) {
        return val == null ? '?' : val;
    }

    function checkmark(bool) {
        return bool ? '✔' : '✗';
    }
</script>

<Nav/>

<main>
    <form on:submit|preventDefault={() => sendChat()}>
        <input type="text" minlength="1" placeholder="Alias" bind:value={alias}/>
        <br/>
        <select on:change={event => message = PRESETS[event.target.value]}>
            {#each Object.entries(PRESETS) as [name, value]}
                <option>{name}</option>
            {/each}
        </select>
        <br/>
        <textarea type="text" minlength="1" placeholder="Message" bind:value={message}/>
        <br/>
        <button>Send (to all players)</button>
    </form>

    <br>
    <br>

    <table>
        <thead>
            <tr>
                <th>ID</th>
                <th>Alias</th>
                <th>Team ID</th>
                <th>Discord ID</th>
                <th>Mod</th>
                <th>Score</th>
                <th>Plays</th>
                <th>Region ID</th>
                <th>IP</th>
                <th>FPS</th>
                <th>RTT</th>
                <th>Msgs.</th>
                <th>Inapp.</th>
                <th>Reports</th>
                <th>Restrict</th>
                <th>Mute</th>
                <th>Chat</th>
                <th>Zeus</th>
            </tr>
        </thead>
        <tbody>
            {#each players as player}
                <tr>
                    <td>{player.player_id}</td>
                    <td class='clickable' on:click={() => overrideAlias(player.player_id, player.alias)}>{player.alias}</td>
                    <td>{player.team_id == null ? '-' : player.team_id}</td>
                    <td>{player.discord_id == null ? '-' : player.discord_id}</td>
                    <td class='clickable' on:click={() => overrideModerator(player.player_id, player.moderator ? false : true)}>{checkmark(player.moderator)}</td>
                    <td>{player.score}</td>
                    <td>{player.plays}</td>
                    <td>{maybe(player.region_id)}</td>
                    <td>{player.ip_address}</td>
                    <td>{maybe(player.fps)}</td>
                    <td>{maybe(player.rtt)}</td>
                    <td>{player.messages}</td>
                    <td>{player.inappropriate_messages}</td>
                    <td>{player.abuse_reports}</td>
                    <td>
                        <select class="mod" on:change|preventDefault={e => restrict(player.player_id, parseInt(e.target.value))} value={player.restriction}>
                            <option disabled>{player.restriction}</option>
                            <option>0</option>
                            <option>5</option>
                            <option>10</option>
                            <option>30</option>
                            <option>60</option>
                            <option>360</option>
                        </select>
                    </td>
                    <td>
                        <select class="mod" on:change|preventDefault={e => mute(player.player_id, parseInt(e.target.value))} value={player.mute}>
                            <option disabled>{player.mute}</option>
                            <option>0</option>
                            <option>5</option>
                            <option>10</option>
                            <option>30</option>
                            <option>60</option>
                            <option>360</option>
                        </select>
                    </td>
                    <td>
                        <button on:click={() => sendChat(player.player_id)}>Send</button>
                    </td>
                    <td>
                        <button>Smite</button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</main>

<style>
    input, select, textarea {
        width: 50%;
    }

    textarea {
        height: 100px;
    }

    select {
        background: initial;
        color: initial;
    }

    select.mod {
        width: max-content;
    }

    .clickable {
        cursor: pointer;
    }
</style>