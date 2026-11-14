"""Generate local development certificates without changing any trust store."""
import ipaddress
from pathlib import Path
import subprocess


def certificate(directory, host):
    address = str(ipaddress.ip_address(host))
    directory = Path(directory)
    directory.mkdir(parents=True, exist_ok=True)
    name = address.replace('.', '_').replace(':', '_')
    cert, key = directory / f'{name}.crt', directory / f'{name}.key'
    if cert.exists() and key.exists():
        return cert, key
    if cert.exists() or key.exists():
        raise RuntimeError('incomplete TLS files exist; preserve or move them before retrying')
    names = ','.join(dict.fromkeys(['DNS:localhost', 'IP:127.0.0.1', f'IP:{address}']))
    subprocess.run(['openssl', 'req', '-x509', '-newkey', 'rsa:2048', '-sha256', '-nodes',
                    '-days', '30', '-keyout', str(key), '-out', str(cert),
                    '-subj', '/CN=X_eTaL local development', '-addext', f'subjectAltName={names}'],
                   check=True, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    key.chmod(0o600)
    return cert, key
