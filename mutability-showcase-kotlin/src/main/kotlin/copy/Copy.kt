package copy

fun updateRequestHandler(request: UpdateRequest) {
    val user = readUser(request.email)
    user?.let {
        val updated = update(user, request)
        store(updated)
    }
}

private fun update(user: User, request: UpdateRequest) =
    user.copy(
        email = request.email,
        name = request.name
    )

private fun store(user: User) {
    // TODO call DB
}

private fun readUser(email: String): User? = when (email) {
    // TODO call DB
    "am@novatec-gmbh.de" -> User(
        email = email,
        name = "Alex"
    )
    else -> null
}

data class UpdateRequest(
    val email: String,
    val name: String
)

private data class User(
    val email: String,
    val name: String
)
