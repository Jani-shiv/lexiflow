# Security Policy & Safeguards

## 1. Password & Sensitive Input Protection
Personal Grammar Enhancer enforces strict safeguards around user credentials and private input:
- **UI Automation (UIA) Password Flag**: Checks `IsPassword` attribute from operating system accessibility APIs.
- **Sensitive Window Detection**: Disables text capture on windows containing sensitive cues such as "Sign In", "Log In", "Master Password", "2FA", "Authenticator", "Windows Security", "User Account Control", or "PIN".
- **Process Blocklisting**: Automatically rejects known credential managers including `1password.exe`, `bitwarden.exe`, `keepass.exe`, `keepassxc.exe`, `lastpass.exe`, `credentialui.exe`, and `consent.exe`.
- **Fail-Safe Principle**: If the security context of an input field cannot be reliably determined, automatic acquisition is disabled.

## 2. Injection Guard Against Feedback Loops
- Programmatic text replacements are wrapped in an `InjectionGuard`.
- Synthetic keystrokes generated during suggestion replacement are tagged and filtered out, preventing the engine from ever re-processing its own output as new user input.

## 3. Clipboard Protection
- When simulating text injection, existing clipboard data is saved to a memory backup before the operation and restored immediately after completion.

## 4. Race Condition & Text Integrity
- Monotonic atomic request IDs (`request_id`) ensure that suggestions generated from older text states are marked stale and discarded if the user continues typing.
