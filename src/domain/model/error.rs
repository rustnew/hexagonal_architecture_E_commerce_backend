use std::fmt;

#[derive(Debug)]
pub enum OrderError {
    TempletesError,
    AuthenticationError,
    ServiceError,
    ReqwestError,
    DatabaseError,
    EmptyCart,
    PaymentRequired,
    AlreadyCompleted,
    UserMismatch,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::TempletesError => write!(f, "Erreur au niveau des templates"),
            OrderError::AuthenticationError => write!(f, "Erreur d'authentification"),
            OrderError::ServiceError => write!(f, "Erreur lors de l'exécution des services"),
            OrderError::ReqwestError => write!(f, "Erreur lors de requêtes HTTP"),
            OrderError::DatabaseError => write!(f, "Erreur au niveau de la base de données"),
            OrderError::EmptyCart => write!(f, "Le panier est vide"),
            OrderError::PaymentRequired => write!(f, "Paiement requis"),
            OrderError::AlreadyCompleted => write!(f, "Commande déjà complétée"),
            OrderError::UserMismatch => write!(f, "L'utilisateur ne correspond pas"),
        }
    }
}

// Définition de AppError comme dans votre exemple original
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    PaymentFailed,
    Forbidden,
    // Ajoutez d'autres variantes au besoin
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::PaymentFailed => write!(f, "Échec du paiement"),
            AppError::Forbidden => write!(f, "Accès refusé"),
        }
    }
}

impl From<OrderError> for AppError {
    fn from(err: OrderError) -> Self {
        match err {
            OrderError::EmptyCart => AppError::BadRequest(err.to_string()),
            OrderError::PaymentRequired => AppError::PaymentFailed,
            OrderError::AlreadyCompleted => AppError::BadRequest(err.to_string()),
            OrderError::UserMismatch => AppError::Forbidden,
            // Conversion par défaut pour les autres erreurs
            _ => AppError::BadRequest(err.to_string()),
        }
    }
}

// Implémentation de std::error::Error pour les deux types d'erreur
impl std::error::Error for OrderError {}
impl std::error::Error for AppError {}