import { useState, useEffect, FC } from 'react';

interface OverdueRental {
  rental_id: number;
  film_title: string;
  customer_name: string;
  rental_date: string;
}

export const ReturnView: FC = () => {
  const [overdue, setOverdue] = useState<OverdueRental[]>([]);
  const [message, setMessage] = useState('');
  const [messageType, setMessageType] = useState<'success' | 'error' | ''>('');
  const [loading, setLoading] = useState(true);

  const fetchOverdue = async () => {
    setLoading(true);
    setMessage('');
    try {
      const response = await fetch('http://dvd-api.local/api/rentals/overdue');
      if (!response.ok) throw new Error('No se pudieron cargar los datos');
      const data: OverdueRental[] = await response.json();
      setOverdue(data);
    } catch (error) {
      console.error(error);
      setMessage('No se pudieron cargar las rentas pendientes.');
      setMessageType('error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchOverdue();
  }, []);

  const handleReturn = async (rentalId: number) => {
    setMessage(`Procesando devolución #${rentalId}...`);
    setMessageType('');
    try {
        const response = await fetch(`http://dvd-api.local/api/rentals/${rentalId}/return`, {
            method: 'PUT',
        });
        if (!response.ok) throw new Error('No se pudo procesar la devolución');
        
        setMessage(`¡Renta #${rentalId} devuelta exitosamente!`);
        setMessageType('success');
        fetchOverdue();
    } catch (error) {
        console.error(error);
        setMessage(`Error al devolver la renta #${rentalId}.`);
        setMessageType('error');
    }
  };

  // --- NUEVA FUNCIÓN PARA CANCELAR ---
  const handleCancel = async (rentalId: number) => {
    if (window.confirm(`¿Estás seguro de que quieres cancelar la renta #${rentalId}? Esta acción no se puede deshacer.`)) {
        setMessage(`Cancelando renta #${rentalId}...`);
        setMessageType('');
        try {
            const response = await fetch(`http://dvd-api.local/api/rentals/${rentalId}`, {
                method: 'DELETE',
            });
            if (!response.ok) throw new Error('No se pudo procesar la cancelación');
            
            setMessage(`¡Renta #${rentalId} cancelada exitosamente!`);
            setMessageType('success');
            fetchOverdue(); // Recargamos la lista
        } catch (error) {
            console.error(error);
            setMessage(`Error al cancelar la renta #${rentalId}.`);
            setMessageType('error');
        }
    }
  };

  return (
    <div className="card">
        <h2>Devolver o Cancelar una Renta</h2>
        <p>Lista de rentas actualmente pendientes de devolución.</p>
        <button onClick={fetchOverdue} disabled={loading}>Refrescar Lista</button>
        {message && <p className={`message ${messageType}`}>{message}</p>}
        {loading ? <p>Cargando...</p> : (
            <table>
                <thead>
                    <tr>
                        <th>ID Renta</th>
                        <th>Título de Película</th>
                        <th>Cliente</th>
                        <th style={{ textAlign: 'center' }}>Acciones</th>
                    </tr>
                </thead>
                <tbody>
                    {overdue.length > 0 ? overdue.map(rental => (
                        <tr key={rental.rental_id}>
                            <td>{rental.rental_id}</td>
                            <td>{rental.film_title}</td>
                            <td>{rental.customer_name}</td>
                            <td>
                                <div className="action-buttons">
                                    <button onClick={() => handleReturn(rental.rental_id)}>Devolver</button>
                                    {/* --- NUEVO BOTÓN DE CANCELAR --- */}
                                    <button className="button-danger" onClick={() => handleCancel(rental.rental_id)}>Cancelar</button>
                                </div>
                            </td>
                        </tr>
                    )) : (
                      <tr>
                        <td colSpan={4}>No hay rentas pendientes.</td>
                      </tr>
                    )}
                </tbody>
            </table>
        )}
    </div>
  );
};