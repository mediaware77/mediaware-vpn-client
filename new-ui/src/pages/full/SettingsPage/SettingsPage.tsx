import './style.scss';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { openUrl } from '@tauri-apps/plugin-opener';
import clsx from 'clsx';
import { type ReactNode, useMemo, useState } from 'react';
import { Divider } from '../../../shared/components/Divider/Divider';
import { FullPageTitle } from '../../../shared/components/FullPageTitle/FullPageTitle';
import { Icon, IconKind } from '../../../shared/components/Icon';
import { Input } from '../../../shared/components/Input/Input';
import { Select } from '../../../shared/components/Select/Select';
import type { SelectOption } from '../../../shared/components/Select/types';
import { SizedBox } from '../../../shared/components/SizedBox/SizedBox';
import { Toggle } from '../../../shared/components/Toggle/Toggle';
import { FullPage } from '../../../shared/layouts/FullPage/FullPage';
import { api } from '../../../shared/rust-api/api';
import {
  getAppConfigQueryOptions,
  getAutoConnectOptionsQueryOptions,
} from '../../../shared/rust-api/query';
import { type AppConfigPatch, LogLevel } from '../../../shared/rust-api/types';
import { ThemeSpacing } from '../../../shared/types';
import { isPresent } from '../../../shared/utils/isPresent';

const LOG_LEVEL_OPTIONS: SelectOption<LogLevel>[] = [
  { key: LogLevel.Off, label: 'Off', value: LogLevel.Off },
  { key: LogLevel.Error, label: 'Error', value: LogLevel.Error },
  { key: LogLevel.Warn, label: 'Warn', value: LogLevel.Warn },
  { key: LogLevel.Info, label: 'Info', value: LogLevel.Info },
  { key: LogLevel.Debug, label: 'Debug', value: LogLevel.Debug },
  { key: LogLevel.Trace, label: 'Trace', value: LogLevel.Trace },
];

export const SettingsPage = () => {
  const queryClient = useQueryClient();
  const { data: appConfig } = useQuery(getAppConfigQueryOptions);
  const { data: autoConnect, isError: autoConnectError } = useQuery(
    getAutoConnectOptionsQueryOptions,
  );
  const [saveError, setSaveError] = useState('');
  const startupOptions: SelectOption<number>[] = [
    { key: 0, label: 'Desligado', value: 0 },
    ...(autoConnect?.locations ?? []).map((location) => ({
      key: location.id,
      label: `${location.instance_name} · ${location.name}`,
      value: location.id,
    })),
  ];
  const selectedStartup = startupOptions.find(
    (option) => option.value === appConfig?.auto_connect_location_id,
  );
  if (appConfig && appConfig.auto_connect_location_id > 0 && !selectedStartup) {
    startupOptions.push({
      key: appConfig.auto_connect_location_id,
      value: appConfig.auto_connect_location_id,
      label: 'Rede indisponível — selecione Desligado',
    });
  }

  const { mutate: patchConfig, isPending: saving } = useMutation({
    mutationFn: (patch: AppConfigPatch) => api.setAppConfig(patch, true),
    onSuccess: (config) => {
      setSaveError('');
      queryClient.setQueryData(getAppConfigQueryOptions.queryKey, config);
    },
    onError: (error) => {
      console.error(error);
      setSaveError(
        'Não foi possível salvar a configuração. Confira se a rede permite conexão sem MFA e tente novamente.',
      );
    },
  });

  const selectedLogLevel = useMemo(
    () =>
      LOG_LEVEL_OPTIONS.find((o) => o.value === appConfig?.log_level) ??
      LOG_LEVEL_OPTIONS[3],
    [appConfig?.log_level],
  );

  if (!appConfig) return null;

  return (
    <FullPage id="settings-page-view">
      <FullPageTitle title="Settings" spacing={ThemeSpacing.Xl} />
      <div className="sections">
        {saveError && <p role="alert">{saveError}</p>}
        {autoConnectError && (
          <p role="alert">Não foi possível consultar as redes para conexão automática.</p>
        )}
        {autoConnect?.supported && (
          <SettingRow
            title="Conectar ao entrar no Windows"
            description="Escolha a rede para conectar em segundo plano, sem código MFA. Só aparecem redes que permitem acesso sem MFA. Quem usar esta conta do Windows poderá usar a VPN. Para parar, selecione Desligado."
          >
            <Select
              testId="windows-auto-connect"
              options={startupOptions}
              value={
                startupOptions.find(
                  (option) => option.value === appConfig.auto_connect_location_id,
                ) ?? startupOptions[0]
              }
              disabled={saving}
              onChange={(option) =>
                patchConfig({ auto_connect_location_id: option.value })
              }
            />
            {autoConnect.locations.length === 0 && (
              <p className="description">
                Nenhuma rede permite conexão automática. Peça ao administrador para
                liberar o acesso sem MFA e atualize o cadastro da instância.
              </p>
            )}
          </SettingRow>
        )}
        <SettingRow
          title="Logging threshold"
          description="Defines the minimum severity level of events that will be recorded in the logs. Only events at or above this level are captured to reduce noise and improve log clarity."
        >
          <Select
            options={LOG_LEVEL_OPTIONS}
            value={selectedLogLevel}
            onChange={(option) => patchConfig({ log_level: option.value })}
          />
        </SettingRow>

        <SettingRow
          title="Session timeout"
          description="If active connection exceeds given time without making an handshake with the server. The connection will be considered invalid and disconnected automatically."
        >
          <Input
            type="number"
            value={appConfig.peer_alive_period}
            onChange={(value) => {
              if (value === null || value === '') return;
              patchConfig({ peer_alive_period: Number(value) });
            }}
          />
        </SettingRow>

        <SettingRow
          title="Maximum Transmission Unit"
          description="MTU sets the largest packet size sent through the network. Lowering it can improve connection stability in restrictive or unreliable ISP networks. The default value on most systems is 1500. Try lowering it to 1300-1400 if you encounter ISP-related issues."
        >
          <Input
            type="number"
            value={appConfig.mtu}
            onChange={(value) => {
              if (value === null || value === '') return;
              patchConfig({ mtu: Number(value) });
            }}
          />
        </SettingRow>

        <SettingRow title="Check for updates automatically" inline>
          <Toggle
            active={appConfig.check_for_updates}
            onClick={() =>
              patchConfig({ check_for_updates: !appConfig.check_for_updates })
            }
          />
        </SettingRow>

        <SettingRow title="Auto start OpenID MFA" inline>
          <Toggle
            active={appConfig.auto_start_openid_mfa}
            onClick={() =>
              patchConfig({ auto_start_openid_mfa: !appConfig.auto_start_openid_mfa })
            }
          />
        </SettingRow>

        <p className="footer">
          PMCG VPN · by Mediaware is based on Defguard and other open-source software
          (AGPL-3.0).{' '}
          <button
            type="button"
            className="link"
            onClick={() =>
              openUrl(
                'https://github.com/mediaware77/mediaware-vpn-client/tree/codex/pmcg-vpn',
              )
            }
          >
            Source code
            <Icon icon={IconKind.OpenInNewWindow} size={14} />
          </button>
        </p>
      </div>
    </FullPage>
  );
};

type SettingRowProps = {
  title: string;
  description?: string;
  children: ReactNode;
  // Render the title and control on a single line instead of stacked.
  inline?: boolean;
  divider?: boolean;
};

const SettingRow = ({
  title,
  description,
  children,
  inline = false,
  divider = true,
}: SettingRowProps) => (
  <div className={clsx('setting-row', { inline })}>
    <div className="head">
      <p className="title">{title}</p>
      {inline && <div className="control">{children}</div>}
    </div>
    {isPresent(description) && (
      <>
        <SizedBox height={ThemeSpacing.Xs} />
        <p className="description">{description}</p>
      </>
    )}
    {!inline && (
      <>
        <SizedBox height={ThemeSpacing.Lg} />
        <div className="control">{children}</div>
      </>
    )}
    {divider && <Divider spacing={ThemeSpacing.Xl} />}
  </div>
);
