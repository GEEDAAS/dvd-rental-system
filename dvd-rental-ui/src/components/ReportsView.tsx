import { useState, useEffect, FormEvent, FC } from 'react';

// --- Interfaces para los datos de los reportes ---
interface MostRentedFilm {
  film_title: string;
  rental_count: number;
}

interface StaffRevenue {
  staff_name: string;
  total_revenue: string;
}

interface CustomerRental {
    rental_id: number;
    film_title: string;
    rental_date: string;
    return_date: string | null;
}

export const ReportsView: FC = () => {
  const [mostRented, setMostRented] = useState<MostRentedFilm[]>([]);
  const [staffRevenue, setStaffRevenue] = useState<StaffRevenue[]>([]);
  const [customerRentals, setCustomerRentals] = useState<CustomerRental[]>([]);
  const [customerId, setCustomerId] = useState('1'); // ID por defecto para buscar
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchGeneralReports = async () => {
      try {
        setLoading(true);
        const [mostRentedRes, staffRevenueRes] = await Promise.all([
          fetch('http://dvd-api.local/api/films/most-rented'),
          fetch('http://dvd-api.local/api/staff/revenue')
        ]);
        setMostRented(await mostRentedRes.json());
        setStaffRevenue(await staffRevenueRes.json());
      } catch (error) {
        console.error("No se pudieron cargar los reportes generales", error);
      } finally {
        setLoading(false);
      }
    };
    fetchGeneralReports();
  }, []);

  const fetchCustomerRentals = async (event: FormEvent) => {
    event.preventDefault();
    const res = await fetch(`http://dvd-api.local/api/customers/${customerId}/rentals`);
    setCustomerRentals(await res.json());
  };

  if (loading) return <div className="card"><p>Cargando reportes...</p></div>;

  return (
    <div className="reports-grid">
        <div className="card">
            <h3>Top 10 Películas Más Rentadas</h3>
            <ol>
                {mostRented.map(film => (
                    <li key={film.film_title}>{film.film_title} ({film.rental_count} rentas)</li>
                ))}
            </ol>
        </div>
        <div className="card">
            <h3>Ingresos por Empleado</h3>
            <ul>
                {staffRevenue.map(staff => (
                    <li key={staff.staff_name}>{staff.staff_name}: ${staff.total_revenue}</li>
                ))}
            </ul>
        </div>
        <div className="card full-width">
            <h3>Historial de Rentas por Cliente</h3>
            <form onSubmit={fetchCustomerRentals}>
                <input type="number" value={customerId} onChange={e => setCustomerId(e.target.value)} placeholder="ID de Cliente" />
                <button type="submit">Buscar</button>
            </form>
             <table>
                <thead>
                    <tr>
                        <th>Título de Película</th>
                        <th>Fecha de Renta</th>
                        <th>Fecha de Devolución</th>
                    </tr>
                </thead>
                <tbody>
                    {customerRentals.map(r => (
                        <tr key={r.rental_id}>
                            <td>{r.film_title}</td>
                            <td>{new Date(r.rental_date).toLocaleDateString()}</td>
                            <td>{r.return_date ? new Date(r.return_date).toLocaleDateString() : 'Pendiente'}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    </div>
  );
};