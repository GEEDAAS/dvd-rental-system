import { useState } from 'react';
import './App.css';
import { RentForm } from './components/RentForm';
import { ReturnView } from './components/ReturnView';
import { ReportsView } from './components/ReportsView';

// Definimos los tipos de vistas que puede tener nuestra aplicación
type View = 'RENT' | 'RETURN' | 'REPORTS';

function App() {
  // Estado para controlar qué vista se está mostrando actualmente
  const [currentView, setCurrentView] = useState<View>('RENT');

  // Función para renderizar el componente correcto basado en el estado
  const renderView = () => {
    switch (currentView) {
      case 'RENT':
        return <RentForm />;
      case 'RETURN':
        return <ReturnView />;
      case 'REPORTS':
        return <ReportsView />;
      default:
        return <RentForm />; // Vista por defecto
    }
  };

  return (
    <div className="container">
      <header>
        <h1>Sistema de Renta de DVDs</h1>
        {/* Navegación para cambiar entre vistas */}
        <nav>
          <button onClick={() => setCurrentView('RENT')} className={currentView === 'RENT' ? 'active' : ''}>Rentar</button>
          <button onClick={() => setCurrentView('RETURN')} className={currentView === 'RETURN' ? 'active' : ''}>Devolver</button>
          <button onClick={() => setCurrentView('REPORTS')} className={currentView === 'REPORTS' ? 'active' : ''}>Reportes</button>
        </nav>
      </header>
      <main>
        {renderView()}
      </main>
    </div>
  );
}

export default App;