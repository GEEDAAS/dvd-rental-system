import { useState, FormEvent, FC } from 'react';

export const RentForm: FC = () => {
  const [customerId, setCustomerId] = useState('');
  const [inventoryId, setInventoryId] = useState('');
  const [staffId, setStaffId] = useState('');
  const [message, setMessage] = useState('');
  const [messageType, setMessageType] = useState<'success' | 'error' | ''>('');

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();
    setMessage('Procesando...');
    setMessageType('');

    try {
      const response = await fetch('http://dvd-api.local/api/rentals', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customer_id: parseInt(customerId),
          inventory_id: parseInt(inventoryId),
          staff_id: parseInt(staffId),
        }),
      });

      if (!response.ok) throw new Error(`La API respondió con estado ${response.status}`);
      
      const result = await response.json();
      setMessage(`¡Renta exitosa! Nuevo ID de Renta: ${result.rental_id}`);
      setMessageType('success');
      setCustomerId('');
      setInventoryId('');
      setStaffId('');
    } catch (error) {
      console.error(error);
      setMessage('Error al crear la renta. Por favor, verifique los IDs.');
      setMessageType('error');
    }
  };

  return (
    <div className="card">
      <h2>Rentar un DVD</h2>
      <form onSubmit={handleSubmit}>
        <input type="number" value={customerId} onChange={(e) => setCustomerId(e.target.value)} placeholder="ID de Cliente" required />
        <input type="number" value={inventoryId} onChange={(e) => setInventoryId(e.target.value)} placeholder="ID de Inventario" required />
        <input type="number" value={staffId} onChange={(e) => setStaffId(e.target.value)} placeholder="ID de Staff" required />
        <button type="submit">Registrar Renta</button>
      </form>
      {message && <p className={`message ${messageType}`}>{message}</p>}
    </div>
  );
};