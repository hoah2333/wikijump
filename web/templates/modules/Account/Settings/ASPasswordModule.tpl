<h1><a href="javascript:;" onclick="Wikijump.modules.AccountModule.utils.loadModule('am-settings')">Account settings</a> / Change password</h1>

<p>
	Below is the form to change your password. Please keep your password private and do not reveal it to
	others. Please also do not make your password a dictionary word easy to guess.
</p>

<div id="password-error" class="error-block" style="display: none"></div>

<form id="change-password-form">
	<table class="form">
		<tr>
			<td>Your current password:</td>
			<td><input class="text" type="password" name="old_password" size="20" maxlength="1024"/></td>
		</tr>
		<tr>
			<td>New password:</td>
			<td>
				<input class="text" type="password" name="new_password1" size="20" maxlength="1024"/>
				<div class="sub">
					Min. 8 characters, max. 1024.
				</div>
			</td>
		</tr>
		<tr>
			<td>New password (repeat):</td>
			<td>
				<input class="text" type="password" name="new_password2" size="20" maxlength="1024"/>
				<div class="sub">
					Please repeat to prevent typos.
				</div>
			</td>
		</tr>
	</table>
	<div class="buttons">
		<input type="button" value="cancel" onclick="Wikijump.modules.AccountModule.utils.loadModule('am-settings')"/>
		<input type="button" value="change" onclick="Wikijump.modules.ASPasswordModule.listeners.save(event)"/>
	</div>
</form>
